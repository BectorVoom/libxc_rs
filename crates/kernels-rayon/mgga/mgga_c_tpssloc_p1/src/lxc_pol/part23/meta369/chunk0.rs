//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1169/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1169(t44620: f64, t974: f64, t43763: f64, t461: f64, t1176: f64, t2402: f64, t42339: f64, t466: f64, t11715: f64, t42341: f64, t11721: f64, t23508: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44621 = t974 * t44620;
    let t44622 = t461 * t43763;
    let t44633 = t2402 * t1176;
    let t44696 = t466 * t42339;
    let t44698 = t44696 * t42341 * t11715;
    let t44701 = t23508 * t11721;
    (t44621, t44622, t44633, t44696, t44698, t44701)
}
