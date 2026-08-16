//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2085/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2085<F: Float>(t136: F, t2457: F, t7929: F, t25944: F, t2470: F, t27887: F, t7284: F, t1955: F, t27836: F, t4075: F, t25934: F, t27865: F, t27869: F, t543: F, t7295: F, t7301: F, t94700: F, t94703: F, t94705: F, t94714: F, t94726: F, t97855: F, t97908: F, t97909: F, t97915: F, t97917: F, t97920: F) -> (F, F, F) {
    let t97922 = t7929 * t136 * t2457;
    let t97923 = t25944 * t97922;
    let t97925 = t27887 * t2470;
    let t97926 = t7284 * t97925;
    let t97933 = t1955 * t27836 * t4075;
    let t97938 = -t97908 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t97909 * t543 + t97915 + F::cast_from(0.17135234354032049604e-2_f64) * t97917 - t97920 + F::cast_from(0.17135234354032049604e-2_f64) * t97923 - F::cast_from(0.96373646535613327357e-2_f64) * t97926 + t94700 - t94703 - F::cast_from(0.17347256376410398924e1_f64) * t94705 * t27865 + F::cast_from(0.8673628188205199462e0_f64) * t97855 * t27869 - F::cast_from(0.17347256376410398924e1_f64) * t97933 * t25934 - F::cast_from(0.14634331517634470219e-1_f64) * t94714 - F::cast_from(0.23131639038696784278e-2_f64) * t94726;
    (t97922, t97925, t97938)
}
