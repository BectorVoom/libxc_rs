//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1989/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1989(t102573: f64, t13739: f64, t1904: f64, t2027: f64, t2028: f64, t25930: f64, t26304: f64, t27868: f64, t28911: f64, t28918: f64, t48020: f64, t49380: f64, t545: f64, t5774: f64, t7295: f64, t7296: f64, t7506: f64, t7511: f64, t94705: f64, t94823: f64, t96512: f64, t96567: f64, t96570: f64, t96577: f64, t96584: f64, t96588: f64, t96591: f64, t97871: f64, t98062: f64) -> f64 {
    let t102700 = 0.13170898365871023197e1_f64 * t7511 * t13739 - 0.17347256376410398924e1_f64 * t27868 * t28911 * t48020 + 0.12851425765524037203e-1_f64 * t96567 + 0.10975748638225852664e-1_f64 * t96570 - 0.19514881078765566038e-1_f64 * t96577 - t96584 + 0.26020884564615598386e1_f64 * t94823 * t26304 * t98062 + 0.17347256376410398924e1_f64 * t25930 * t28911 * t97871 - 0.65854491829355115987e0_f64 * t96512 * t1904 - 0.17347256376410398924e1_f64 * t94705 * t28918 + 0.25702851531048074406e-1_f64 * t96588 + 0.17347256376410398924e1_f64 * t7295 * t7296 * t7506 * t5774 + 0.4336814094102599731e0_f64 * t27868 * t26304 * t49380 - 0.4336814094102599731e0_f64 * t2027 * t2028 * t545 * t102573 + t96591;
    t102700
}
