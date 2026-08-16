//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1050/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1050(t22085: f64, t22112: f64, t225: f64, t68: f64, t484: f64, t1177: f64, t21749: f64, t1196: f64, t20217: f64, t974: f64, t11848: f64, t20234: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22113 = t22085 + t22112;
    let t22114 = t22113 * t225;
    let t22115 = t22114 * t68;
    let t22116 = t22115 * t484;
    let t22119 = t1177 * t21749;
    let t22128 = t1196 * t20217;
    let t22129 = t974 * t22128;
    let t22132 = t11848 * t20234;
    (t22113, t22114, t22116, t22119, t22129, t22132)
}
