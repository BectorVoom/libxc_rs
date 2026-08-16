//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1284/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1284(t1955: f64, t23359: f64, t106395: f64, t106407: f64, t106423: f64, t106431: f64, t106433: f64, t1949: f64, t1959: f64, t231: f64, t23384: f64, t23413: f64, t23414: f64, t27199: f64, t29655: f64, t29675: f64, t6016: f64, t7053: f64, t7070: f64, t7076: f64, t7759: f64, t93118: f64, t93334: f64, t99425: f64, t99435: f64) -> f64 {
    let t113373 = t1955 * t23359;
    let t113380 = -0.68549505033305214441e-2_f64 * t99425 - 0.65854491829355115987e0_f64 * t7053 * t23384 + 0.29272321618148349057e-1_f64 * t106395 + 0.10408353825846239354e2_f64 * t7070 * t93118 * t1949 * t23413 + 0.26020884564615598386e1_f64 * t27199 * t29655 + 0.34697458558045176417e-2_f64 * t99435 - 0.29272321618148349057e-1_f64 * t106407 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t7759 * t6016 * t231 + 0.13010442282307799193e1_f64 * t27199 * t29675 + 0.32927245914677557992e-1_f64 * t106423 - 0.4336814094102599731e0_f64 * t113373 * t1959 - t93334 + 0.38554277296572111609e-1_f64 * t106431 - 0.21684070470512998656e-1_f64 * t106433 - 0.39512695097613069591e1_f64 * t7053 * t23414;
    t113380
}
