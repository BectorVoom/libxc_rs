//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2285/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2285(t1036: f64, t28572: f64, t1015: f64, t1022: f64, t17841: f64, t1935: f64, t23564: f64, t23604: f64, t25645: f64, t25652: f64, t25658: f64, t25679: f64, t28558: f64, t28582: f64, t28587: f64, t3032: f64, t343: f64, t360: f64, t4649: f64, t5872: f64, t6730: f64, t6734: f64, t7583: f64, t82911: f64, t88341: f64, t88362: f64, t88367: f64, t88385: f64, t88537: f64) -> f64 {
    let t99590 = t28572 * t1036;
    let t99600 = -t88341 - 0.20186378047070195428e-3_f64 * t25652 * t25658 * t23604 * t4649 + 0.10093189023535097714e-3_f64 * t82911 * t28582 - 0.10093189023535097714e-3_f64 * t1935 * t17841 * t343 * t6734 - 0.10093189023535097714e-3_f64 * t6730 * t28558 - t88385 - 0.20186378047070195428e-3_f64 * t88362 * t7583 - 0.20186378047070195428e-3_f64 * t88367 * t7583 - 0.20186378047070195428e-3_f64 * t25645 * t25679 + t99590 / 2304.0_f64 + 0.10093189023535097714e-3_f64 * t88537 * t1015 * t5872 * t3032 * t1022 * t360 - 0.10093189023535097714e-3_f64 * t23564 * t28587;
    t99600
}
