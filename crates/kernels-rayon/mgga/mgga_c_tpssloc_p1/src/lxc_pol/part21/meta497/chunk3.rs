//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2115/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2115(t17079: f64, t17108: f64, t2752: f64, t5660: f64, t13105: f64, t16685: f64, t16688: f64, t16691: f64, t16692: f64, t16695: f64, t16696: f64, t1877: f64, t193: f64, t202: f64, t4303: f64, t4307: f64, t868: f64, t870: f64, t9789: f64, t9793: f64, t9797: f64, t9820: f64, t9824: f64, t9876: f64, t9884: f64, t9887: f64, t9890: f64) -> (f64, f64, f64) {
    let t17109 = t17079 + t17108;
    let t17116 = t5660 * t2752;
    let t17119 = t17109 * t193 * t202 * t870 - t17116 * t1877 * t868 - 2.0_f64 * t1877 * t4303 * t4307 + t13105 + t16685 + t16688 + t16691 + t16692 + t16695 + t16696 - t9789 + t9793 + t9797 - t9820 - t9824 - t9876 - t9884 + t9887 + t9890;
    (t17109, t17116, t17119)
}
