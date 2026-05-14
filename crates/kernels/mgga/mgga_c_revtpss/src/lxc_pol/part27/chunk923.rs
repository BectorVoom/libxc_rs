//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 923/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk923<F: Float>(t12327: F, t12328: F, t406: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t1134: F, t3390: F) -> (F, F, F, F) {
    let t12329 = t12327 * t12328;
    let t12331 = 1.0/pow_3_2(t406);
    let t12332 = t12331 * t12328;
    let t12334 = 0.5477111111111111111e-1 * t12252 + 0.36514074074074074075e-1 * t12259 + 0.27385555555555555556e0 * t12261 - 0.16431333333333333333e0 * t12263 - 0.32862666666666666666e0 * t12265 - 0.16431333333333333333e0 * t12271 + 0.49293999999999999999e0 * t12275 + 0.82156666666666666667e-1 * t12279 - 0.82156666666666666668e-1 * t12284 + 0.49293999999999999999e0 * t12289 - 0.59793333333333333333e0 * t12292 + 0.1898925e1 * t12323 + 0.142419375e1 * t12329 - 0.76790625e-1 * t12332;
    let t12343 = t3390 * t1134;
    (t12329, t12332, t12334, t12343)
}
