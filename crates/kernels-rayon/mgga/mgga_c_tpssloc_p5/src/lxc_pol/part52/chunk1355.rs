//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1355/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1355(t31246: f64, t7688: f64, t1388: f64, t7752: f64, t26161: f64, t26162: f64, t33082: f64, t6876: f64, t1983: f64, t55242: f64, t8493: f64, t33133: f64, t6880: f64) -> (f64, f64, f64, f64, f64) {
    let t120692 = t31246 * t7688;
    let t120694 = t7752 * t1388;
    let t120697 = 4.0_f64 * t26161 * t26162 * t120694;
    let t120699 = 2.0_f64 * t6876 * t33082;
    let t120702 = 2.0_f64 * t1983 * t8493 * t55242;
    let t120703 = t33133 * t6880;
    (t120692, t120697, t120699, t120702, t120703)
}
