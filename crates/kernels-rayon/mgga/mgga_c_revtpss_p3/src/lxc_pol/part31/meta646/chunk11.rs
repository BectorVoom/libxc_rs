//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2123/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2123(t27216: f64, t27279: f64, t27213: f64, t6022: f64, t886: f64, t29674: f64, t689: f64, t25431: f64, t25411: f64, t14587: f64, t18324: f64, t18615: f64, t1949: f64, t231: f64, t25322: f64, t25391: f64, t2718: f64, t27267: f64, t27353: f64, t27357: f64, t6072: f64, t7053: f64, t7070: f64, t7076: f64, t7759: f64, t7766: f64, t93206: f64, t93207: f64, t93210: f64, t93224: f64, t99274: f64) -> f64 {
    let t106216 = t27216 * t27279;
    let t106218 = t27213 * t27279;
    let t106228 = t6022 * t886;
    let t106235 = t29674 * t689;
    let t106236 = t25431 * t106235;
    let t106238 = t25411 * t106235;
    let t106245 = -0.25702851531048074406e-1_f64 * t106216 + 0.14456046980341999104e-1_f64 * t106218 + t99274 - 0.8673628188205199462e0_f64 * t7766 * t27267 + 0.13170898365871023197e1_f64 * t7053 * t18324 + t93206 - 0.17347256376410398924e1_f64 * t27353 * t2718 * t7759 * t14587 + 0.17347256376410398924e1_f64 * t25391 * t27357 * t106228 - 0.65854491829355115987e0_f64 * t25322 * t6072 - 0.13009920719177044025e-1_f64 * t93207 - t93210 + t93224 - 0.72280234901709995518e-2_f64 * t106236 + 0.12851425765524037203e-1_f64 * t106238 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t1949 * t18615 * t231;
    t106245
}
