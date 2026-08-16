//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 994/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk994(t5266: f64, t558: f64, t71910: f64, t2367: f64, t8264: f64, t118: f64, t76242: f64, t27055: f64, t77335: f64, t5148: f64, t551: f64, t14444: f64, t1587: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t77979 = 0.11974241701863808564e0_f64 * t5266 * t71910 * t558;
    let t77980 = t8264 * t2367;
    let t77982 = 0.39914139006212695214e-1_f64 * t118 * t77980;
    let t77983 = 0.68186654135613354325e-2_f64 * t76242;
    let t77988 = 0.35922725105591425692e0_f64 * t27055 * t77335;
    let t77992 = 0.11974241701863808564e0_f64 * t5148 * t71910 * t551;
    let t77995 = 0.11974241701863808564e0_f64 * t5148 * t14444 * t1587;
    (t77979, t77980, t77982, t77983, t77988, t77992, t77995)
}
