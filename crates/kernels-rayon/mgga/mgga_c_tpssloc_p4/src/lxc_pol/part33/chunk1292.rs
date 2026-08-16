//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1292/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1292(t23384: f64, t28496: f64, t225: f64, t28488: f64, t28557: f64, t381: f64, t3173: f64, t5919: f64, t1921: f64, t28702: f64, t82431: f64, t28510: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99230 = t23384 * t28496;
    let t99248 = t28488 * t225;
    let t99273 = t28557 * t381;
    let t99296 = t3173 * t5919;
    let t99297 = t1921 * t99296;
    let t99301 = t82431 * t28702;
    let t99330 = t23384 * t28510;
    (t99230, t99248, t99273, t99297, t99301, t99330)
}
