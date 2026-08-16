//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1212/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1212(t32783: f64, t6876: f64, t1845: f64, t6995: f64, t26161: f64, t26162: f64, t31537: f64, t7468: f64, t31540: f64, t26003: f64, t8526: f64, t24995: f64, t37593: f64, t5308: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t119831 = t6876 * t32783;
    let t119832 = t1845 * t6995;
    let t119835 = 4.0_f64 * t26161 * t26162 * t119832;
    let t119837 = 4.0_f64 * t31537 * t7468;
    let t119839 = 4.0_f64 * t31540 * t7468;
    let t119841 = 4.0_f64 * t8526 * t26003;
    let t119844 = 6.0_f64 * t24995 * t37593 * t5308;
    (t119831, t119835, t119837, t119839, t119841, t119844)
}
