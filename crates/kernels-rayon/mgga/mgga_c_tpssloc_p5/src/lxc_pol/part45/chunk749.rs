//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 749/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk749(t1891: f64, t22822: f64, t133: f64, t6601: f64, t6590: f64, t6604: f64, t13229: f64, t232: f64, t815: f64, t22813: f64, t22816: f64, t1895: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23093 = t22822 * t1891;
    let t23094 = t23093 * t133;
    let t23095 = t23094 * t6601;
    let t23096 = 0.52708876011794399171e-3_f64 * t23095;
    let t23097 = t6590 * t6604;
    let t23098 = t13229 * t232;
    let t23099 = t815 * t23098;
    let t23100 = t23097 * t23099;
    let t23102 = t22813 * t1891;
    let t23103 = t23102 * t22816;
    let t23104 = t794 * t1895;
    (t23094, t23095, t23096, t23097, t23098, t23100, t23103, t23104)
}
