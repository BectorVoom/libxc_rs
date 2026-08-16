//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 690/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk690(t12350: f64, t798: f64, t15: f64, t944: f64, t1014: f64, t142: f64, t3088: f64, t5: f64, t119: f64, t955: f64, t1049: f64, t213: f64, t5816: f64, t5823: f64, t5827: f64) -> (f64, f64, f64, f64, f64) {
    let t12351 = 1.0_f64 / t12350;
    let t12352 = t798 * t12351;
    let t12407 = t15 * t944;
    let t12408 = t1014 * t12407;
    let t12410 = t5 * t142 * t3088;
    let t12414 = t5 * t119 * t955;
    let t12425 = 0.35867157975189532869e-1_f64 * t213 - 0.13661666666666666667e-1_f64 * t5827 + 0.38744444444444444446e-2_f64 * t5816 - 0.15538616723388920628e-3_f64 * t1049 + 0.18204739583333333333e-4_f64 * t5823;
    (t12352, t12408, t12410, t12414, t12425)
}
