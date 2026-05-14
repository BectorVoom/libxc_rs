//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1082/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1082<F: Float>(t26571: F, t26602: F, t26615: F, t26597: F, t26576: F, t26607: F, t26611: F, t7583: F, t92241: F, t92256: F, t92258: F, t92260: F, t92263: F, t92266: F, t92268: F, t92271: F, t92273: F, t92276: F) -> (F,) {
    let t92278 = t26602 * t26571;
    let t92280 = t26602 * t26615;
    let t92282 = t26597 * t26571;
    let t92284 = t26607 * t26576;
    let t92286 = t26607 * t26611;
    let t92288 = t92241 * t7583;
    let t92290 = t26597 * t26611;
    let t92292 = -0.8347923046875e-3 * t92256 - 0.41703125000000000001e-2 * t92258 + 0.12985658072916666667e-2 * t92260 - 0.16217881944444444444e-1 * t92263 + 0.48653645833333333332e-2 * t92266 - 0.48653645833333333332e-2 * t92268 + 0.208515625e-2 * t92271 + 0.208515625e-2 * t92273 + 0.2782641015625e-3 * t92276 - 0.41703125000000000001e-2 * t92278 + 0.208515625e-2 * t92280 + 0.97307291666666666666e-2 * t92282 - 0.83479230468750000001e-3 * t92284 + 0.2782641015625e-3 * t92286 - 0.97307291666666666666e-2 * t92288 - 0.48653645833333333332e-2 * t92290;
    (t92292,)
}
