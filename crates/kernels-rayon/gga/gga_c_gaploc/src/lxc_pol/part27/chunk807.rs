//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 807/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk807(t2634: f64, t4614: f64, t1890: f64, t7291: f64, t590: f64, t5241: f64, t739: f64, t7068: f64, t2582: f64, t4673: f64, t1457: f64, t7132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7656 = t4614 * t2634;
    let t7659 = t1890 * t7291;
    let t7660 = t7659 * t590;
    let t7663 = t5241 * t7291;
    let t7664 = t7663 * t590;
    let t7667 = t739 * t7291;
    let t7668 = t7667 * t590;
    let t7671 = t739 * t7068;
    let t7672 = t7671 * t590;
    let t7675 = t1890 * t7068;
    let t7676 = t7675 * t590;
    let t7679 = t4673 * t2582;
    let t7682 = t1457 * t7132;
    (t7656, t7659, t7660, t7664, t7667, t7668, t7671, t7672, t7676, t7679, t7682)
}
