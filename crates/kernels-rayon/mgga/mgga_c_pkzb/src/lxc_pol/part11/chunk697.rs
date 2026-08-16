//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 697/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk697(t1597: f64, t470: f64, t57: f64, t1517: f64, t490: f64, t1600: f64, t74: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4998 = 1.0_f64 / t1597 / t470;
    let t4999 = t57 * t4998;
    let t5000 = t1517 * t490;
    let t5002 = 1.0_f64 / t1600 / t74;
    let t5003 = t5000 * t5002;
    let t5005 = 0.51726012919273400301e3_f64 * t4999 * t5003;
    (t4998, t4999, t5000, t5002, t5003, t5005)
}
