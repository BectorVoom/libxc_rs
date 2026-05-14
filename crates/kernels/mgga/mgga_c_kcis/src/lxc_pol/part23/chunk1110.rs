//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1110/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1110<F: Float>(t16783: F, t5780: F, t7923: F, t1394: F, t16737: F, t27351: F, t27369: F, t5426: F, t94227: F, t94626: F, t98242: F, t98304: F, t98308: F, t98313: F, t98315: F, t98322: F, t98328: F, t98331: F, t98334: F) -> (F, F, F) {
    let t98337 = t5780 * t7923 * t16783;
    let t98340 = t1394 * t7923 * t16737;
    let t98342 = -0.18550940104166666667e-3 * t27369 * t98304 + 0.51485339506172839507e-4 * t98308 - 0.1492375e-1 * t98313 + 0.61782407407407407408e-3 * t94626 * t98315 * t5426 * t27351 - 0.61836467013888888888e-4 * t94227 * t98322 - 0.18550940104166666667e-3 * t94227 * t98242 - 0.66327777777777777776e-2 * t98328 - 0.33163888888888888888e-2 * t98331 - 0.16581944444444444444e-1 * t98334 + 0.13265555555555555555e-1 * t98337 - 0.3684876543209876543e-3 * t98340;
    (t98337, t98340, t98342)
}
