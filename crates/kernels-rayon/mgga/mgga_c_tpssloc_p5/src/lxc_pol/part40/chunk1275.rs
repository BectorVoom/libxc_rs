//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1275/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1275(t30164: f64, t8138: f64, t1444: f64, t29922: f64, t659: f64, t29926: f64, t2585: f64, t656: f64, t2: f64, t29894: f64, t29896: f64, t29898: f64, t29901: f64, t29903: f64, t30147: f64, t30149: f64, t30152: f64, t30156: f64, t30159: f64, t30162: f64, t8128: f64, t8137: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30165 = t8138 * t30164;
    let t30168 = t29922 * t1444;
    let t30171 = t1444 * t659;
    let t30172 = t29926 * t30171;
    let t30175 = t2585 * t656;
    let t30176 = t8138 * t2;
    let t30179 = -t29894 - 2.0_f64 / 3.0_f64 * t29896 - 5.0_f64 / 9.0_f64 * t29898 + 5.0_f64 / 9.0_f64 * t29901 - 2.0_f64 / 3.0_f64 * t30147 - 3.0_f64 / 4.0_f64 * t29903 * t30149 - 5.0_f64 / 12.0_f64 * t8128 * t30152 + 5.0_f64 / 12.0_f64 * t8128 * t30156 + t8128 * t30159 / 4.0_f64 + 5.0_f64 / 9.0_f64 * t30162 + 5.0_f64 / 12.0_f64 * t8128 * t30165 + 25.0_f64 / 72.0_f64 * t8137 * t30168 - 5.0_f64 / 36.0_f64 * t8137 * t30172 - 5.0_f64 / 24.0_f64 * t30175 * t30176;
    (t30165, t30168, t30171, t30172, t30175, t30176, t30179)
}
