//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1280/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1280(t113192: f64, t113206: f64, t113219: f64, t113240: f64, t5977: f64, t7759: f64, t106151: f64, t106153: f64, t106275: f64, t113163: f64, t1949: f64, t1956: f64, t1957: f64, t231: f64, t23244: f64, t233: f64, t25391: f64, t25392: f64, t25416: f64, t27199: f64, t2723: f64, t27353: f64, t27357: f64, t29644: f64, t7070: f64, t7076: f64, t76106: f64, t7770: f64, t93142: f64, t99186: f64, t99188: f64, t99202: f64, t99206: f64, t99212: f64) -> (f64, f64, f64) {
    let t113242 = t113192 + t113206 + t113219 + t113240;
    let t113261 = t7759 * t5977;
    let t113267 = -t93142 - 0.26020884564615598386e1_f64 * t25391 * t25392 * t113163 - 0.43368140941025997312e-1_f64 * t106151 + 0.77108554593144223218e-1_f64 * t106153 + 0.39029762157531132076e-1_f64 * t99186 + 0.21951497276451705329e-1_f64 * t99188 - 0.4336814094102599731e0_f64 * t1956 * t1957 * t233 * t113242 + 0.51405703062096148812e-1_f64 * t99202 + 0.26020884564615598386e1_f64 * t106275 * t7770 - 0.72280234901709995519e-3_f64 * t99206 - 0.26020884564615598386e1_f64 * t27353 * t27357 * t76106 - 0.78062653693846795158e1_f64 * t27199 * t29644 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t1949 * t23244 * t231 - 0.26020884564615598386e1_f64 * t7070 * t25416 * t113261 * t2723 - 0.10281140612419229763e-1_f64 * t99212;
    (t113242, t113261, t113267)
}
