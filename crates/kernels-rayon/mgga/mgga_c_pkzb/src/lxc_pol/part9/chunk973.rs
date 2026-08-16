//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 973/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk973(t2105: f64, t7682: f64, t1120: f64, t2057: f64, t2104: f64, t276: f64, t2895: f64, t2899: f64, t2922: f64, t2933: f64, t5646: f64, t5661: f64, t5666: f64, t5984: f64, t735: f64, t7621: f64, t7630: f64, t7632: f64, t7639: f64, t7642: f64, t7650: f64, t7655: f64, t7660: f64, t7664: f64, t7668: f64, t7673: f64, t7678: f64) -> (f64, f64) {
    let t7683 = t2105 * t7682;
    let t7686 = t7621 / 432.0_f64 - 11.0_f64 / 108.0_f64 * t2057 * t1120 + t735 * t2895 / 18.0_f64 - t7630 - t276 * t7632 / 96.0_f64 + 0.45732285992607719436e-2_f64 * t5984 * t2933 - t7639 + 0.12862205435420921092e-2_f64 * t2104 * t7642 - t5646 / 288.0_f64 + t5661 / 54.0_f64 + t5666 / 144.0_f64 - 0.42874018118069736972e-3_f64 * t2104 * t7650 - 0.42874018118069736972e-3_f64 * t2922 * t7655 - 0.21437009059034868486e-3_f64 * t2922 * t7660 + 0.21437009059034868486e-3_f64 * t7664 * t7668 - 0.85748036236139473944e-3_f64 * t2104 * t7673 - 0.42874018118069736972e-3_f64 * t2104 * t7678 - 0.85748036236139473944e-3_f64 * t2899 * t7683;
    (t7683, t7686)
}
