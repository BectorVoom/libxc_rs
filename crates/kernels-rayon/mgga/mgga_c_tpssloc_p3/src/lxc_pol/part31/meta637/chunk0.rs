//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1904/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1904(t28192: f64, t80727: f64, t1307: f64, t1377: f64, t22633: f64, t22635: f64, t6460: f64, t1842: f64, t26331: f64, t26337: f64, t26189: f64, t26193: f64, t6888: f64) -> (f64, f64, f64, f64) {
    let t97664 = t80727 * t28192;
    let t97705 = t22633 * t22635 * t1377 * t6460 * t1307;
    let t97721 = t1842 * t1307;
    let t97724 = t26331 * t22635 * t26337 * t97721;
    let t97729 = t6888 * t26193 * t26189;
    (t97664, t97705, t97724, t97729)
}
