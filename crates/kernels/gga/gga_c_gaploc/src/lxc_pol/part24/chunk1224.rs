//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1224/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1224<F: Float>(t32339: F, t1843: F, t24478: F, t7064: F, t10669: F, t10674: F, t1908: F, t1935: F, t1939: F, t270: F, t29304: F, t29310: F, t29324: F, t32314: F, t32329: F, t32332: F, t32334: F, t32337: F, t3434: F, t3452: F, t681: F, t738: F) -> F {
    let t32340 = F::cast_from(0.32043859292259267849e-3_f64) * t32339;
    let t32342 = t7064 * t1843 * t24478;
    let t32343 = F::cast_from(0.32043859292259267849e-3_f64) * t32342;
    let t32344 = t29304 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t738 * t32314 - F::cast_from(0.15381052460284448567e-1_f64) * t681 * t10669 + F::cast_from(0.76905262301422242837e-2_f64) * t1935 * t3434 + F::cast_from(0.15381052460284448567e-1_f64) * t681 * t10674 - F::cast_from(0.20508069947045931424e-1_f64) * t1939 * t3452 + F::cast_from(0.34180116578409885707e-2_f64) * t1908 * t3434 + t29310 - t32329 - t32332 + t32334 + t32337 + t32340 - t29324 + t32343;
    t32344
}
