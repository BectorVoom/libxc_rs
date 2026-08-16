//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1185/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1185(t27: f64, t799: f64, t89: f64, t90304: f64, t5225: f64, t43524: f64, t5299: f64, t2681: f64, t71238: f64, t83718: f64, t83720: f64, t83722: f64, t89865: f64, t89868: f64, t89872: f64, t89875: f64, t89879: f64, t89883: f64, t89887: f64, t89891: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90307 = t89 * t27 * t799 * t90304;
    let t90308 = t5225 * t5225;
    let t90311 = t89 * t27 * t43524 * t90308;
    let t90313 = t5299 * t5299;
    let t90316 = t89 * t27 * t2681 * t90313;
    let t90322 = 40.0_f64 / 9.0_f64 * t89865 - 12.0_f64 * t89868 + 40.0_f64 / 27.0_f64 * t89872 - 20.0_f64 / 9.0_f64 * t89875 + 4.0_f64 / 3.0_f64 * t89879 - 8.0_f64 / 3.0_f64 * t89883 - 4.0_f64 * t89887 - 16.0_f64 / 3.0_f64 * t89891 - t90307 + 24.0_f64 * t90311 + 6.0_f64 * t90316 - 4.0_f64 / 3.0_f64 * t83718 - 4.0_f64 / 3.0_f64 * t83720 + 8.0_f64 / 9.0_f64 * t83722 + 16.0_f64 / 3.0_f64 * t71238;
    (t90307, t90308, t90311, t90313, t90316, t90322)
}
