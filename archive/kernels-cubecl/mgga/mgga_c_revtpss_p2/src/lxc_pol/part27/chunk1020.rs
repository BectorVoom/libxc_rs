//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1020/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1020<F: Float>(t409: F, t416: F, t1134: F, t3391: F, t406: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F) -> (F, F, F) {
    let t12327 = F::cast_from(1.0_f64) / t409 / t416 / F::cast_from(4.0_f64);
    let t12328 = t3391 * t1134;
    let t12329 = t12327 * t12328;
    let t12331 = F::cast_from(1.0_f64)/pow_3_2::<F>(t406);
    let t12332 = t12331 * t12328;
    let t12334 = F::cast_from(0.5477111111111111111e-1_f64) * t12252 + F::cast_from(0.36514074074074074075e-1_f64) * t12259 + F::cast_from(0.27385555555555555556e0_f64) * t12261 - F::cast_from(0.16431333333333333333e0_f64) * t12263 - F::cast_from(0.32862666666666666666e0_f64) * t12265 - F::cast_from(0.16431333333333333333e0_f64) * t12271 + F::cast_from(0.49293999999999999999e0_f64) * t12275 + F::cast_from(0.82156666666666666667e-1_f64) * t12279 - F::cast_from(0.82156666666666666668e-1_f64) * t12284 + F::cast_from(0.49293999999999999999e0_f64) * t12289 - F::cast_from(0.59793333333333333333e0_f64) * t12292 + F::cast_from(0.1898925e1_f64) * t12323 + F::cast_from(0.142419375e1_f64) * t12329 - F::cast_from(0.76790625e-1_f64) * t12332;
    (t12329, t12332, t12334)
}
