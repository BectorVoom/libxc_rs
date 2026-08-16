//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 696/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk696(t3791: f64, t562: f64, t550: f64, t6976: f64, t1992: f64, t6914: f64, t6979: f64, t3734: f64, t6968: f64, t6637: f64, t22685: f64, t6546: f64, t6887: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22740 = t562 * t3791;
    let t22741 = t22740 * t550;
    let t22742 = t6976 * t22741;
    let t22743 = t1992 * t22742;
    let t22745 = t6914 * t6979;
    let t22746 = 0.38381794893125283518e-1_f64 * t22745;
    let t22747 = t6968 * t3734;
    let t22748 = t6637 * t22747;
    let t22749 = t22685 * t22748;
    let t22751 = t6546 * t6887;
    (t22740, t22743, t22745, t22746, t22749, t22751)
}
