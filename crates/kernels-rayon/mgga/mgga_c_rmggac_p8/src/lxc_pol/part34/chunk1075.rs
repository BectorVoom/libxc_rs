//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1075/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1075(t78450: f64, t15530: f64, t4965: f64, t15496: f64, t2160: f64, t638: f64, t739: f64, t8264: f64, t8975: f64, t76041: f64, t76043: f64, t639: f64, t640: f64, t9595: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t78451 = 0.15961724959986689774e-4_f64 * t78450;
    let t78454 = 0.39914139006212695214e-1_f64 * t4965 * t15530;
    let t78456 = t638 * t2160 * t15496;
    let t78457 = 0.40650199722100037752e-3_f64 * t78456;
    let t78462 = 0.11974241701863808564e0_f64 * t739 * t8264 * t8975;
    let t78464 = 0.2553875993597870364e-4_f64 * t76041;
    let t78465 = 0.14967802127329760705e-1_f64 * t76043;
    let t78468 = t638 * t639 * t640 * t9595;
    (t78451, t78454, t78457, t78462, t78464, t78465, t78468)
}
