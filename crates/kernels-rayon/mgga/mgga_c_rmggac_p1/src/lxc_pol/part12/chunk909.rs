//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 909/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk909(t7255: f64, t8422: f64, t2289: f64, t35384: f64, t35262: f64, t35285: f64, t39663: f64, t39667: f64, t39672: f64, t39676: f64, t39679: f64, t39682: f64, t39686: f64, t39690: f64, t39694: f64, t39698: f64, t39702: f64, t39706: f64, t534: f64, t72: f64, t7884: f64) -> f64 {
    let t39709 = t7255 * t8422;
    let t39711 = t35384 * t2289;
    let t39713 = -0.20455996240684006296e-1_f64 * t39663 + 0.54549323308490683457e-1_f64 * t39667 - 0.79828278012425390426e-1_f64 * t35262 + 0.13637330827122670864e0_f64 * t39672 + 0.27274661654245341728e-1_f64 * t39676 + t39679 + 0.13637330827122670864e-1_f64 * t39682 + 0.6818665413561335432e-1_f64 * t39686 - 0.40911992481368012592e-1_f64 * t39690 + 0.21819729323396273382e0_f64 * t39694 + 0.54549323308490683456e-1_f64 * t39698 - t39702 + t72 * t534 * t7884 - 0.20455996240684006296e-1_f64 * t39706 + 0.59590439850616975158e-4_f64 * t35285 + 0.85129199786595678796e-5_f64 * t39709 - 0.12769379967989351819e-4_f64 * t39711;
    t39713
}
