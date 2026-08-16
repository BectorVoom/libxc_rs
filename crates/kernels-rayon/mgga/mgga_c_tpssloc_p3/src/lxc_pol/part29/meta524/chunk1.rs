//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1902/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1902(t1983: f64, t26149: f64, t6880: f64, t7685: f64, t6876: f64, t7754: f64, t1982: f64, t8944: f64) -> (f64, f64, f64, f64) {
    let t26150 = t1983 * t26149;
    let t26153 = 3.0_f64 * t7685 * t6880;
    let t26157 = t6876 * t7754;
    let t26161 = t1982 * t8944;
    (t26150, t26153, t26157, t26161)
}
