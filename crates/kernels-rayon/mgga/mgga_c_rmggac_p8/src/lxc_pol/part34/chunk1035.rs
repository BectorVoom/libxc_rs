//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1035/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1035(t25854: f64, t77327: f64, t27176: f64, t77330: f64, t3851: f64, t71982: f64, t8642: f64, t71983: f64, t8646: f64, t72023: f64, t8650: f64, t71951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77933 = 0.35922725105591425692e0_f64 * t25854 * t77327;
    let t77935 = 0.47896966807455234256e0_f64 * t27176 * t77330;
    let t77937 = t3851 * t71982 * t8642;
    let t77938 = 0.20455996240684006296e-1_f64 * t77937;
    let t77939 = t71983 * t8646;
    let t77940 = 0.40911992481368012592e-1_f64 * t77939;
    let t77941 = t72023 * t8650;
    let t77942 = 0.20455996240684006296e-1_f64 * t77941;
    let t77943 = 0.79828278012425390426e-1_f64 * t71951;
    (t77933, t77935, t77938, t77940, t77942, t77943)
}
