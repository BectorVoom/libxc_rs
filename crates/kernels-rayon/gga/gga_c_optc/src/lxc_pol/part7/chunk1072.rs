//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1072/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1072(t106: f64, t167: f64, t2100: f64, t2106: f64, t2107: f64, t2189: f64, t22217: f64, t22775: f64, t22843: f64, t22905: f64, t22911: f64, t22915: f64, t22922: f64, t22933: f64, t22934: f64, t22942: f64, t22984: f64, t23024: f64, t23080: f64, t23123: f64, t23159: f64, t23196: f64, t23233: f64, t23274: f64, t3461: f64, t670: f64, t6964: f64, t6976: f64, t6978: f64, t6982: f64, t6983: f64, t708: f64, t7138: f64, t9804: f64) -> f64 {
    let t23281 = 0.27818116767324025134e1_f64 * t106 * (t22217 + t22775 + t22843 + t22905) * t167 - 0.11127246706929610054e2_f64 * t106 * t22911 * t708 + 0.33381740120788830161e2_f64 * t106 * t22915 * t2107 - 0.1669087006039441508e2_f64 * t106 * t6964 * t2189 - 0.66763480241577660323e2_f64 * t106 * t22922 * t6978 + 0.66763480241577660323e2_f64 * t9804 * t6983 - 0.11127246706929610054e2_f64 * t106 * t2100 * t7138 + 0.6676348024157766032e2_f64 * t106 * t22933 * t22934 - 0.10014522036236649048e3_f64 * t3461 * t6976 * t2107 * t2189 + 0.16690870060394415081e2_f64 * t106 * t2106 * t22942 + 0.22254493413859220108e2_f64 * t3461 * t6982 * t7138 - 0.27818116767324025134e1_f64 * t106 * t670 * (t22984 + t23024 + t23080 + t23123 + t23159 + t23196 + t23233 + t23274);
    t23281
}
