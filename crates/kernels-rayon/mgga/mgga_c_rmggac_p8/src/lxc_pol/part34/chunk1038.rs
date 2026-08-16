//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1038/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1038(t14444: f64, t1652: f64, t8940: f64, t5266: f64, t558: f64, t71910: f64, t2367: f64, t8264: f64, t118: f64, t76242: f64, t27055: f64, t77335: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77976 = 0.11974241701863808564e0_f64 * t8940 * t14444 * t1652;
    let t77979 = 0.11974241701863808564e0_f64 * t5266 * t71910 * t558;
    let t77980 = t8264 * t2367;
    let t77982 = 0.39914139006212695214e-1_f64 * t118 * t77980;
    let t77983 = 0.68186654135613354325e-2_f64 * t76242;
    let t77988 = 0.35922725105591425692e0_f64 * t27055 * t77335;
    (t77976, t77979, t77980, t77982, t77983, t77988)
}
