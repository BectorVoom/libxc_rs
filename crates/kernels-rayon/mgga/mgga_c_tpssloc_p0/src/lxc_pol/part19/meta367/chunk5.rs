//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1349/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1349(t42790: f64, t42824: f64, t42860: f64, t42899: f64, t42933: f64, t42966: f64, t43034: f64, t43079: f64, t225: f64, t10427: f64, t13969: f64, t3130: f64) -> (f64, f64, f64) {
    let t43082 = t42790 + t42824 + t42860 + t42899 + t42933 + t42966 + t43034 + t43079;
    let t43083 = t43082 * t225;
    let t43094 = t3130 * t13969 * t10427;
    (t43082, t43083, t43094)
}
