//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2533/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2533(t13542: f64, t13779: f64, t2986: f64, t13546: f64, t13555: f64, t13784: f64, t13528: f64, t1592: f64, t42891: f64, t973: f64, t13812: f64, t13822: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t48384 = t2986 * t13779 * t13542;
    let t48387 = t2986 * t13779 * t13546;
    let t48390 = t2986 * t13784 * t13555;
    let t48394 = t2986 * t13784 * t13528;
    let t48397 = t973 * t42891 * t1592;
    let t48402 = t973 * t13822 * t13812;
    (t48384, t48387, t48390, t48394, t48397, t48402)
}
