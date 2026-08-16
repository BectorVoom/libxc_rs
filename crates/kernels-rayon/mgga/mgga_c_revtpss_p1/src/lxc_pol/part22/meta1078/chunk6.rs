//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3866/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3866(t22022: f64, t9775: f64, t22061: f64, t808: f64, t9845: f64, t13920: f64, t4003: f64, t22085: f64, t9962: f64, t22182: f64, t47215: f64, t22046: f64, t22079: f64, t3829: f64, t3936: f64, t46730: f64, t46951: f64, t48573: f64, t48577: f64, t48591: f64, t48593: f64, t5671: f64, t5673: f64, t5674: f64, t6849: f64, t800: f64, t9840: f64) -> (f64, f64) {
    let t74299 = t9775 * t22022;
    let t74304 = t9845 * t808 * t22061;
    let t74314 = t4003 * t13920;
    let t74319 = t9962 * t22085;
    let t74322 = t47215 * t22182;
    let t74329 = -0.76220476654346199061e-4_f64 * t74299 + 0.85748036236139473944e-4_f64 * t48573 - 0.85748036236139473944e-4_f64 * t48577 + 0.25410001404642664112e-5_f64 * t74304 - 0.17149607247227894789e-2_f64 * t5671 * t3936 * t22046 * t46951 + 0.42874018118069736972e-3_f64 * t5671 * t5673 * t22079 * t9840 + 0.85748036236139473944e-3_f64 * t5671 * t5673 * t5674 * t74314 + 0.20007875121765877254e-2_f64 * t74319 + 0.20007875121765877254e-2_f64 * t48591 - 0.27104001498285508387e-3_f64 * t74322 + 5.0_f64 / 4.0_f64 * t46730 * t800 * t6849 * t3829 + 0.16006300097412701803e-1_f64 * t48593;
    (t74314, t74329)
}
