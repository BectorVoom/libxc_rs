//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2167/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2167(t15030: f64, t25319: f64, t25391: f64, t25392: f64, t25426: f64, t27199: f64, t7053: f64, t93206: f64, t93207: f64, t93210: f64, t93224: f64, t93226: f64, t93228: f64, t93231: f64, t93349: f64, t99274: f64, t99277: f64, t99287: f64, t99289: f64) -> f64 {
    let t99295 = 0.4336814094102599731e0_f64 * t27199 * t25426 + t99274 + 0.26341796731742046394e1_f64 * t7053 * t15030 + t93206 + 0.26020884564615598386e1_f64 * t93349 * t25392 * t99277 - 0.2601984143835408805e-1_f64 * t93207 - t93210 + t93224 - 0.72280234901709995518e-2_f64 * t93226 + 0.12851425765524037203e-1_f64 * t93228 - t99287 - 0.8673628188205199462e0_f64 * t25391 * t25392 * t99289 - t93231 - 0.26020884564615598386e1_f64 * t27199 * t25319;
    t99295
}
