//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1221/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1221(t32339: f64, t1843: f64, t24478: f64, t7064: f64, t10669: f64, t10674: f64, t1908: f64, t1935: f64, t1939: f64, t270: f64, t29304: f64, t29310: f64, t29324: f64, t32314: f64, t32329: f64, t32332: f64, t32334: f64, t32337: f64, t3434: f64, t3452: f64, t681: f64, t738: f64) -> f64 {
    let t32340 = 0.32043859292259267849e-3_f64 * t32339;
    let t32342 = t7064 * t1843 * t24478;
    let t32343 = 0.32043859292259267849e-3_f64 * t32342;
    let t32344 = t29304 - 0.76905262301422242837e-2_f64 * t270 * t738 * t32314 - 0.15381052460284448567e-1_f64 * t681 * t10669 + 0.76905262301422242837e-2_f64 * t1935 * t3434 + 0.15381052460284448567e-1_f64 * t681 * t10674 - 0.20508069947045931424e-1_f64 * t1939 * t3452 + 0.34180116578409885707e-2_f64 * t1908 * t3434 + t29310 - t32329 - t32332 + t32334 + t32337 + t32340 - t29324 + t32343;
    t32344
}
