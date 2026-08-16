//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2184/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2184(t12283: f64, t19962: f64, t19882: f64, t19996: f64, t3866: f64, t40018: f64, t6371: f64, t12189: f64, t6375: f64, t40138: f64, t6396: f64, t19951: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56933 = t12283 * t19962;
    let t56935 = t12283 * t19882;
    let t56937 = t3866 * t19996;
    let t56946 = t40018 * t6371;
    let t56953 = t12189 * t6375;
    let t56959 = t40138 * t6396;
    let t56961 = t12283 * t19951;
    (t56933, t56935, t56937, t56946, t56953, t56959, t56961)
}
