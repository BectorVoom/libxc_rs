//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1211/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1211(t119733: f64, t119783: f64, t31222: f64, t7685: f64, t24987: f64, t8494: f64, t26142: f64, t8526: f64, t2314: f64, t32677: f64, t4034: f64, t5107: f64, t652: f64, t8326: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t119784 = t119733 + t119783;
    let t119795 = t7685 * t31222;
    let t119796 = t24987 * t8494;
    let t119810 = 4.0_f64 * t8526 * t26142;
    let t119824 = 2.0_f64 * t2314 * t32677;
    let t119826 = 2.0_f64 * t4034 * t32677;
    let t119830 = 2.0_f64 * t652 * t5107 * t8326;
    (t119784, t119795, t119796, t119810, t119824, t119826, t119830)
}
