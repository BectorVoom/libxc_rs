//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3143/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3143(t1174: f64, t15281: f64, t18554: f64, t11570: f64, t17635: f64, t11569: f64, t1177: f64, t1178: f64, t15390: f64, t18321: f64, t3443: f64, t3447: f64, t3457: f64, t3461: f64, t3475: f64, t460: f64, t4919: f64, t4934: f64, t52066: f64, t52100: f64, t52224: f64, t52228: f64, t52240: f64, t52250: f64, t55677: f64, t6138: f64) -> f64 {
    let t65041 = t1174 * t15281 * t18554;
    let t65056 = t11570 * t17635;
    let t65073 = -0.55555555555555555554e-3_f64 * t65041 - 0.27777777777777777777e-3_f64 * t1174 * t1177 * t1178 * t55677 + 0.33333333333333333333e-2_f64 * t3447 * t4919 * t52224 - 0.44444444444444444444e-2_f64 * t3447 * t15390 * t52228 + 0.17283950617283950617e-2_f64 * t3447 * t52100 * t52066 - 0.74074074074074074072e-3_f64 * t3447 * t11569 * t65056 + 0.98765432098765432094e-3_f64 * t52240 - 0.11111111111111111111e-2_f64 * t52250 - 0.83333333333333333332e-3_f64 * t1174 * t4934 * t6138 * t3475 * t460 - 0.27160493827160493827e-2_f64 * t18321 * t3461 - 0.54320987654320987654e-2_f64 * t18321 * t3457 + 0.36213991769547325103e-2_f64 * t18321 * t3443;
    t65073
}
