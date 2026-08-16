//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1245/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1245(t1394: f64, t27364: f64, t5655: f64, t17005: f64, t7923: f64, t17010: f64, t4153: f64, t16783: f64, t5780: f64, t16737: f64, t27351: f64, t27369: f64, t5426: f64, t94227: f64, t94626: f64, t98242: f64, t98304: f64, t98308: f64, t98313: f64, t98315: f64, t98322: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98328 = t1394 * t27364 * t5655;
    let t98331 = t1394 * t7923 * t17005;
    let t98334 = t4153 * t7923 * t17010;
    let t98337 = t5780 * t7923 * t16783;
    let t98340 = t1394 * t7923 * t16737;
    let t98342 = -0.18550940104166666667e-3_f64 * t27369 * t98304 + 0.51485339506172839507e-4_f64 * t98308 - 0.1492375e-1_f64 * t98313 + 0.61782407407407407408e-3_f64 * t94626 * t98315 * t5426 * t27351 - 0.61836467013888888888e-4_f64 * t94227 * t98322 - 0.18550940104166666667e-3_f64 * t94227 * t98242 - 0.66327777777777777776e-2_f64 * t98328 - 0.33163888888888888888e-2_f64 * t98331 - 0.16581944444444444444e-1_f64 * t98334 + 0.13265555555555555555e-1_f64 * t98337 - 0.3684876543209876543e-3_f64 * t98340;
    (t98328, t98331, t98334, t98337, t98340, t98342)
}
