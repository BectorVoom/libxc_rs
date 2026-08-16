//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 941/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk941(t531: f64, t571: f64, t193: f64, t2423: f64, t2426: f64, t2486: f64, t3734: f64, t3816: f64, t3819: f64, t3821: f64, t3823: f64, t3825: f64, t3828: f64, t3830: f64, t3832: f64, t3834: f64, t3836: f64) -> f64 {
    let t3924 = t531 * t571;
    let t3928 = 6.0_f64 * t193 * t3734 * t3924 - t2423 - t2426 - t2486 - t3816 + t3819 + t3821 - t3823 + t3825 + t3828 - t3830 - t3832 + t3834 + t3836;
    t3928
}
