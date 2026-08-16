//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1864/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1864<F: Float>(t94522: F, t94525: F, t94568: F, t94570: F, t7284: F, t96282: F, t26277: F, t94913: F, t25944: F, t96259: F, t1385: F, t7506: F) -> (F, F, F, F, F, F, F, F) {
    let t96341 = F::cast_from(0.15117061203111996147e0_f64) * t94522;
    let t96342 = F::cast_from(0.80328230880474379779e-6_f64) * t94525;
    let t96358 = F::cast_from(0.45178982497454656792e-6_f64) * t94568;
    let t96359 = F::cast_from(0.28900264064772933812e-2_f64) * t94570;
    let t96374 = F::cast_from(0.22487184191643109717e-1_f64) * t7284 * t96282;
    let t96380 = t94913 * t26277;
    let t96382 = t25944 * t96259;
    let t96392 = t1385 * t7506;
    (t96341, t96342, t96358, t96359, t96374, t96380, t96382, t96392)
}
