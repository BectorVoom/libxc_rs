//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1232/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1232(t40201: f64, t40215: f64, t40217: f64, t40220: f64, t40222: f64, t40228: f64, t40197: f64, t40204: f64, t40207: f64, t40210: f64, t40213: f64, t40224: f64) -> f64 {
    let t41743 = 0.19043987679069580389e-1_f64 * t40201;
    let t41748 = 0.19514881078765566037e-1_f64 * t40215;
    let t41749 = 0.21951497276451705328e-1_f64 * t40217;
    let t41750 = 0.45022119329691164871e0_f64 * t40220;
    let t41751 = 0.46230515946956099004e0_f64 * t40222;
    let t41753 = 0.32524801797942610062e-3_f64 * t40228;
    let t41754 = -0.65854491829355115984e0_f64 * t40197 - t41743 + 0.5200933044032561138e0_f64 * t40204 - 0.5200933044032561138e0_f64 * t40207 + 0.21951497276451705328e0_f64 * t40210 - 0.20803732176130244552e1_f64 * t40213 - t41748 - t41749 + t41750 - t41751 - 0.87327386630866483588e-2_f64 * t40224 + t41753;
    t41754
}
