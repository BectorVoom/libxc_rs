//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2085/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2085(t136: f64, t2457: f64, t7929: f64, t25944: f64, t2470: f64, t27887: f64, t7284: f64, t1955: f64, t27836: f64, t4075: f64, t25934: f64, t27865: f64, t27869: f64, t543: f64, t7295: f64, t7301: f64, t94700: f64, t94703: f64, t94705: f64, t94714: f64, t94726: f64, t97855: f64, t97908: f64, t97909: f64, t97915: f64, t97917: f64, t97920: f64) -> (f64, f64, f64) {
    let t97922 = t7929 * t136 * t2457;
    let t97923 = t25944 * t97922;
    let t97925 = t27887 * t2470;
    let t97926 = t7284 * t97925;
    let t97933 = t1955 * t27836 * t4075;
    let t97938 = -t97908 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t97909 * t543 + t97915 + 0.17135234354032049604e-2_f64 * t97917 - t97920 + 0.17135234354032049604e-2_f64 * t97923 - 0.96373646535613327357e-2_f64 * t97926 + t94700 - t94703 - 0.17347256376410398924e1_f64 * t94705 * t27865 + 0.8673628188205199462e0_f64 * t97855 * t27869 - 0.17347256376410398924e1_f64 * t97933 * t25934 - 0.14634331517634470219e-1_f64 * t94714 - 0.23131639038696784278e-2_f64 * t94726;
    (t97922, t97925, t97938)
}
