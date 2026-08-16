//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 991/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk991(t10915: f64, t22242: f64, t43598: f64, t2684: f64, t43486: f64, t7585: f64, t43750: f64, t43752: f64, t43754: f64, t43757: f64, t43759: f64, t43761: f64, t43762: f64, t43766: f64, t43768: f64, t43771: f64, t43774: f64, t43775: f64, t43776: f64, t43777: f64, t43778: f64, t43781: f64, t43783: f64, t43787: f64) -> f64 {
    let t43790 = 0.21450293971110256001e1_f64 * t22242 * t10915 * t43598;
    let t43793 = 0.87421871174939309262e2_f64 * t2684 * t7585 * t43486;
    let t43794 = -t43750 - t43752 + t43754 + t43757 - t43759 + t43761 + 0.23833659967900284447e0_f64 * t43762 + t43766 + 0.85801175884441024008e1_f64 * t43768 - 0.42900587942220512004e1_f64 * t43771 + t43774 + t43775 - t43776 + t43777 - t43778 + t43781 - t43783 - t43787 + t43790 + t43793;
    t43794
}
