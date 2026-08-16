//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 989/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk989(t31631: f64, t6897: f64, t794: f64, t113981: f64, t113987: f64, t113964: f64, t113966: f64, t113969: f64, t113972: f64, t113975: f64, t113978: f64, t113983: f64, t113985: f64, t113989: f64, t113993: f64, t113997: f64) -> (f64, f64) {
    let t115439 = t6897 * t794 * t31631;
    let t115447 = 0.13457585364713463618e-3_f64 * t113981;
    let t115450 = 7.0_f64 / 144.0_f64 * t113987;
    let t115454 = t113964 / 96.0_f64 + 0.22608743412718618878e-1_f64 * t113966 + 0.32298204875312312682e-2_f64 * t113969 - 0.16149102437656156341e-2_f64 * t113972 - 0.16149102437656156341e-2_f64 * t113975 + 0.19378922925187387609e-1_f64 * t113978 - t115447 - t113983 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t113985 + t115450 - t113989 / 192.0_f64 + 0.67826230238155856632e-1_f64 * t113993 - 0.96894614625936938046e-2_f64 * t113997;
    (t115439, t115454)
}
