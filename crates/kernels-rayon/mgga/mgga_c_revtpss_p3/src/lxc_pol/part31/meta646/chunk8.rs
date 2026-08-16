//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2120/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2120(t6017: f64, t886: f64, t29668: f64, t689: f64, t25431: f64, t25411: f64, t14495: f64, t25391: f64, t25392: f64, t27189: f64, t27312: f64, t27349: f64, t27353: f64, t4487: f64, t93151: f64, t93158: f64, t93161: f64, t99186: f64, t99188: f64, t99202: f64, t99206: f64, t99334: f64) -> f64 {
    let t106143 = t6017 * t886;
    let t106150 = t29668 * t689;
    let t106151 = t25431 * t106150;
    let t106153 = t25411 * t106150;
    let t106164 = -0.17347256376410398924e1_f64 * t25391 * t99334 * t27312 + 0.8673628188205199462e0_f64 * t27353 * t99334 * t14495 - 0.8673628188205199462e0_f64 * t25391 * t25392 * t106143 - 0.17347256376410398924e1_f64 * t25391 * t99334 * t27349 - 0.14456046980341999104e-1_f64 * t106151 + 0.25702851531048074406e-1_f64 * t106153 - 0.96373646535613327357e-2_f64 * t93151 + 0.26019841438354088051e-1_f64 * t99186 + 0.14634331517634470219e-1_f64 * t99188 + 0.17135234354032049604e-2_f64 * t93158 - 0.22849835011101738147e-2_f64 * t93161 + 0.34270468708064099208e-1_f64 * t99202 + 0.26341796731742046394e1_f64 * t27189 * t4487 - 0.4818682326780666368e-3_f64 * t99206;
    t106164
}
