//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 976/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk976(t24990: f64, t6878: f64, t1983: f64, t192: f64, t531: f64, t1982: f64, t5308: f64, t8945: f64, t111: f64, t7450: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24991 = t6878 * t24990;
    let t24993 = 3.0_f64 * t1983 * t24991;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t24996 = t8945 * t5308;
    let t24998 = 6.0_f64 * t24995 * t24996;
    let t24999 = t7450 * t111;
    (t24991, t24993, t24994, t24995, t24996, t24998, t24999)
}
