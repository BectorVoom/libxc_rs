//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1397/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1397(t1220: f64, t3289: f64, t7274: f64, t3274: f64, t9236: f64, t11786: f64, t1188: f64, t1221: f64, t26115: f64, t26122: f64, t26150: f64, t26152: f64, t26156: f64, t277: f64, t27831: f64, t3245: f64, t3290: f64, t4281: f64, t4282: f64, t4289: f64, t4290: f64, t8410: f64, t914: f64, t9244: f64, t95: f64) -> f64 {
    let t27837 = t1220 * t7274 * t3289;
    let t27839 = t3274 * t9236;
    let t27841 = -4.0_f64 * t1220 * t914 * t1221 * t26115 + 8.0_f64 / 3.0_f64 * t11786 * t9244 - 4.0_f64 / 3.0_f64 * t4281 * t3245 * t4282 * t26122 + 8.0_f64 / 9.0_f64 * t4281 * t4289 * t4290 * t26122 - 2.0_f64 * t8410 * t3290 + 0.25844881434903430496e-2_f64 * t95 * t277 * t27831 * t1188 - t26150 + 4.0_f64 / 9.0_f64 * t27837 - 4.0_f64 / 3.0_f64 * t27839 + t26152 - t26156;
    t27841
}
