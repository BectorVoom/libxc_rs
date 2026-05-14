//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 520/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk520<F: Float>(t1048: F, t2867: F, t795: F, t2271: F, t970: F, t1463: F, t1470: F, t1480: F, t1488: F, t1529: F, t1533: F, t2328: F, t2490: F, t2492: F, t2494: F, t2495: F) -> (F, F, F) {
    let t2869 = t1048 * t2867 * t795;
    let t2872 = t2271 * t970;
    let t2879 = t2490 + t2492 + t1470 - t1480 - t1488 - t2494 + t2328 - t1529 + t1463 + t2495 - t1533;
    (t2869, t2872, t2879)
}
