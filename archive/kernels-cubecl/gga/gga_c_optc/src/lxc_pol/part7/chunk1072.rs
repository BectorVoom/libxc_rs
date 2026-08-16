//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1072/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1072<F: Float>(t106: F, t167: F, t2100: F, t2106: F, t2107: F, t2189: F, t22217: F, t22775: F, t22843: F, t22905: F, t22911: F, t22915: F, t22922: F, t22933: F, t22934: F, t22942: F, t22984: F, t23024: F, t23080: F, t23123: F, t23159: F, t23196: F, t23233: F, t23274: F, t3461: F, t670: F, t6964: F, t6976: F, t6978: F, t6982: F, t6983: F, t708: F, t7138: F, t9804: F) -> F {
    let t23281 = F::cast_from(0.27818116767324025134e1_f64) * t106 * (t22217 + t22775 + t22843 + t22905) * t167 - F::cast_from(0.11127246706929610054e2_f64) * t106 * t22911 * t708 + F::cast_from(0.33381740120788830161e2_f64) * t106 * t22915 * t2107 - F::cast_from(0.1669087006039441508e2_f64) * t106 * t6964 * t2189 - F::cast_from(0.66763480241577660323e2_f64) * t106 * t22922 * t6978 + F::cast_from(0.66763480241577660323e2_f64) * t9804 * t6983 - F::cast_from(0.11127246706929610054e2_f64) * t106 * t2100 * t7138 + F::cast_from(0.6676348024157766032e2_f64) * t106 * t22933 * t22934 - F::cast_from(0.10014522036236649048e3_f64) * t3461 * t6976 * t2107 * t2189 + F::cast_from(0.16690870060394415081e2_f64) * t106 * t2106 * t22942 + F::cast_from(0.22254493413859220108e2_f64) * t3461 * t6982 * t7138 - F::cast_from(0.27818116767324025134e1_f64) * t106 * t670 * (t22984 + t23024 + t23080 + t23123 + t23159 + t23196 + t23233 + t23274);
    t23281
}
