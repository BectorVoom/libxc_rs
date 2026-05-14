//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1165/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1165<F: Float>(t113192: F, t113206: F, t113219: F, t113240: F, t5977: F, t7759: F, t106151: F, t106153: F, t106275: F, t113163: F, t1949: F, t1956: F, t1957: F, t231: F, t23244: F, t233: F, t25391: F, t25392: F, t25416: F, t27199: F, t2723: F, t27353: F, t27357: F, t29644: F, t7070: F, t7076: F, t76106: F, t7770: F, t93142: F, t99186: F, t99188: F, t99202: F, t99206: F, t99212: F) -> (F, F, F) {
    let t113242 = t113192 + t113206 + t113219 + t113240;
    let t113261 = t7759 * t5977;
    let t113267 = -t93142 - 0.26020884564615598386e1 * t25391 * t25392 * t113163 - 0.43368140941025997312e-1 * t106151 + 0.77108554593144223218e-1 * t106153 + 0.39029762157531132076e-1 * t99186 + 0.21951497276451705329e-1 * t99188 - 0.4336814094102599731e0 * t1956 * t1957 * t233 * t113242 + 0.51405703062096148812e-1 * t99202 + 0.26020884564615598386e1 * t106275 * t7770 - 0.72280234901709995519e-3 * t99206 - 0.26020884564615598386e1 * t27353 * t27357 * t76106 - 0.78062653693846795158e1 * t27199 * t29644 + 0.4336814094102599731e0 * t7070 * t7076 * t1949 * t23244 * t231 - 0.26020884564615598386e1 * t7070 * t25416 * t113261 * t2723 - 0.10281140612419229763e-1 * t99212;
    (t113242, t113261, t113267)
}
