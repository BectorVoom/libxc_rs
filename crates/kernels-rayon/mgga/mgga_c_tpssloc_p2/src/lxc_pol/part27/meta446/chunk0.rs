//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1781/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1781(t252: f64, t2631: f64, t2632: f64, t22996: f64, t1888: f64, t6579: f64, t6649: f64, t232: f64, t6646: f64, t1902: f64, t2627: f64, t2633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22997 = t252 * t2631;
    let t22998 = t22997 * t2632;
    let t22999 = t22996 * t22998;
    let t23000 = t1888 * t22999;
    let t23002 = t6579 * t6649;
    let t23003 = 0.38381794893125283518e-1_f64 * t23002;
    let t23004 = t22997 * t232;
    let t23005 = t6646 * t23004;
    let t23006 = t1888 * t23005;
    let t23008 = t2627 * t1902;
    let t23009 = t23008 * t2633;
    (t22998, t22999, t23000, t23002, t23003, t23004, t23005, t23006, t23009)
}
