//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1194/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1194<F: Float>(t1168: F, t3471: F, t3479: F, t1156: F, t3451: F, t1169: F, t12430: F, t12252: F, t12259: F, t12261: F, t12263: F, t12265: F, t12271: F, t12275: F, t12279: F, t12284: F, t12289: F, t12292: F, t12323: F, t12329: F, t12332: F) -> (F, F, F, F) {
    let t12508 = t3471 * t3479 * t1168;
    let t12511 = t1156 * t3451;
    let t12514 = t12430 * t1169;
    let t12531 = F::cast_from(0.5519e-1_f64) * t12252 + F::cast_from(0.36793333333333333333e-1_f64) * t12259 + F::cast_from(0.27595e0_f64) * t12261 - F::cast_from(0.16557e0_f64) * t12263 - F::cast_from(0.33114e0_f64) * t12265 - F::cast_from(0.16557e0_f64) * t12271 + F::cast_from(0.49671e0_f64) * t12275 + F::cast_from(0.82785e-1_f64) * t12279 - F::cast_from(0.82785e-1_f64) * t12284 + F::cast_from(0.49671e0_f64) * t12289 - F::cast_from(0.60384999999999999999e0_f64) * t12292 + F::cast_from(0.258925e1_f64) * t12323 + F::cast_from(0.19419375e1_f64) * t12329 - F::cast_from(0.412621875e-1_f64) * t12332;
    (t12508, t12511, t12514, t12531)
}
