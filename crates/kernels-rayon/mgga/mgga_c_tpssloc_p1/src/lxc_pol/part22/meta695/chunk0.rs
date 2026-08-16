//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2275/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2275(t1213: f64, t18941: f64, t248: f64, t3570: f64, t15730: f64, t5019: f64, t3508: f64, t6218: f64, t1215: f64, t11721: f64, t6224: f64, t15594: f64, t4993: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65424 = t1213 * t248 * t3570 * t18941;
    let t65444 = t5019 * t15730;
    let t65464 = t6218 * t3508;
    let t65469 = t6218 * t1215;
    let t65474 = t6224 * t11721;
    let t65479 = t15594 * t4993;
    (t65424, t65444, t65464, t65469, t65474, t65479)
}
