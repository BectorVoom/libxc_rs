//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1638/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1638<F: Float>(t15403: F, t3447: F, t14736: F, t4900: F, t14740: F, t14731: F, t11575: F, t4904: F, t134: F, t3439: F, t461: F, t4724: F) -> (F, F, F, F, F, F) {
    let t15405 = F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t15403;
    let t15406 = t4900 * t14736;
    let t15409 = t4900 * t14740;
    let t15412 = t4900 * t14731;
    let t15415 = t11575 * t4904;
    let t15418 = t134 * t3439;
    let t15419 = t15418 * t461;
    let t15420 = t15419 * t4724;
    (t15405, t15406, t15409, t15412, t15415, t15420)
}
