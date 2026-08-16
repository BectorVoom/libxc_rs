//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 949/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk949(t1987: f64, t45561: f64, t1990: f64, t1979: f64, t1982: f64, t458: f64, t9734: f64, t674: f64, t7715: f64, t9774: f64, t1997: f64, t1734: f64, t2124: f64) -> (f64, f64, f64, f64, f64) {
    let t45757 = t45561 * t1987;
    let t45759 = t45561 * t1990;
    let t45763 = t9734 * t458 * t1979 * t1982;
    let t45766 = t9774 * t7715 * t674;
    let t45767 = t45766 * t1997;
    let t45769 = t2124 * t1734;
    (t45757, t45759, t45763, t45767, t45769)
}
