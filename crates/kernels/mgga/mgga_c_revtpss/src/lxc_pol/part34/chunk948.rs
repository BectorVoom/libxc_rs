//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 948/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk948<F: Float>(t16840: F, t6474: F, t24220: F, t3435: F, t12248: F, t5071: F, t6449: F, t5087: F, t12254: F, t24228: F, t141: F, t1145: F, t24244: F, t16706: F, t16876: F, t20276: F, t20278: F, t20280: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F) -> (F, F, F, F, F, F, F) {
    let t24261 = 0.48245938496077605201e2 * t16840 * t6474;
    let t24262 = t24220 * t3435;
    let t24264 = 0.96491876992155210402e2 * t12248 * t24262;
    let t24265 = t5071 * t6449;
    let t24267 = t5087 * t6449;
    let t24271 = t12254 * t24228;
    let t24272 = t141 * t24271;
    let t24274 = t1145 * t24244;
    let t24275 = t141 * t24274;
    let t24285 = -0.28483875e1 * t24265 + 0.46074375e0 * t24267 + 0.39862222222222222223e0 * t16706 + 0.27385555555555555556e0 * t16876 + 0.36514074074074074075e-1 * t24272 + 0.49293999999999999999e0 * t24275 + 0.5477111111111111111e-1 * t20276 - 0.32862666666666666666e0 * t20278 - 0.16431333333333333333e0 * t20280 + 0.19931111111111111111e0 * t20283 - 0.59793333333333333333e0 * t20285 - 0.29896666666666666667e0 * t20287 + 0.33218518518518518518e0 * t24230 - 0.11958666666666666667e1 * t24234;
    (t24261, t24264, t24265, t24267, t24272, t24275, t24285)
}
