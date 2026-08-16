//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 817/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk817(t10012: f64, t8669: f64, t2101: f64, t2925: f64, t313: f64, t769: f64, t9014: f64, t321: f64, t1: f64, t10810: f64, t2021: f64, t22623: f64, t8502: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t24549 = t10012 * t8669;
    let t24660 = t2101 * t2925;
    let t24661 = t313 * t24660;
    let t24799 = t769 * t9014;
    let t24884 = t321 * t2925;
    let t24885 = t24884 * t1;
    let t24968 = t2021 * t10810;
    let t25070 = t22623 * t8502;
    (t24549, t24660, t24661, t24799, t24884, t24885, t24968, t25070)
}
