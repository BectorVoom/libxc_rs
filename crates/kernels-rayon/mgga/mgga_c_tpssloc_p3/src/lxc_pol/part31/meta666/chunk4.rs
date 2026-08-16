//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1959/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1959(t19289: f64, t19451: f64, t1983: f64, t2039: f64, t2095: f64, t2314: f64, t24987: f64, t24995: f64, t26114: f64, t26161: f64, t26179: f64, t26558: f64, t26875: f64, t27150: f64, t27171: f64, t27219: f64, t27226: f64, t29197: f64, t29211: f64, t35259: f64, t4028: f64, t4034: f64, t4072: f64, t5308: f64, t57806: f64, t6468: f64, t652: f64, t671: f64, t7057: f64, t7166: f64, t7458: f64, t7802: f64, t7890: f64, t7941: f64, t96830: f64, t97890: f64) -> f64 {
    let t101091 = -4.0_f64 * t7458 * t27219 - 4.0_f64 * t4028 * t27150 - 2.0_f64 * t652 * t29197 * t671 - 2.0_f64 * t19451 * t7057 + 12.0_f64 * t24995 * t35259 * t5308 + t7166 * t6468 - 4.0_f64 * t652 * t7890 * t4072 + 4.0_f64 * t26161 * t26558 * t96830 - 4.0_f64 * t7458 * t27171 - 2.0_f64 * t2314 * t29211 - 2.0_f64 * t4034 * t29211 - 2.0_f64 * t652 * t19289 * t2039 - t1983 * t2095 * t57806 + 2.0_f64 * t24987 * t7941 + 12.0_f64 * t97890 * t26875 - 4.0_f64 * t26114 * t7802 - 4.0_f64 * t26179 * t7802 - 4.0_f64 * t7458 * t27226 - 4.0_f64 * t4028 * t27219;
    t101091
}
