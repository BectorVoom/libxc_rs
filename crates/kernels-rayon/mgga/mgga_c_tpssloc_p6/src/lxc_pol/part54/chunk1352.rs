//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1352/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1352(t55353: f64, t8657: f64, t1983: f64, t24990: f64, t31758: f64, t24991: f64, t8607: f64, t4026: f64, t8595: f64, t1442: f64, t31518: f64, t22574: f64, t31299: f64, t33899: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120871 = 27.0_f64 * t55353 * t8657;
    let t120874 = 3.0_f64 * t1983 * t31758 * t24990;
    let t120876 = 3.0_f64 * t8607 * t24991;
    let t120877 = t4026 * t8595;
    let t120878 = t1442 * t31518;
    let t120881 = 3.0_f64 * t22574 * t33899 * t31299;
    (t120871, t120874, t120876, t120877, t120878, t120881)
}
