//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3866/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3866<F: Float>(t22022: F, t9775: F, t22061: F, t808: F, t9845: F, t13920: F, t4003: F, t22085: F, t9962: F, t22182: F, t47215: F, t22046: F, t22079: F, t3829: F, t3936: F, t46730: F, t46951: F, t48573: F, t48577: F, t48591: F, t48593: F, t5671: F, t5673: F, t5674: F, t6849: F, t800: F, t9840: F) -> (F, F) {
    let t74299 = t9775 * t22022;
    let t74304 = t9845 * t808 * t22061;
    let t74314 = t4003 * t13920;
    let t74319 = t9962 * t22085;
    let t74322 = t47215 * t22182;
    let t74329 = -F::cast_from(0.76220476654346199061e-4_f64) * t74299 + F::cast_from(0.85748036236139473944e-4_f64) * t48573 - F::cast_from(0.85748036236139473944e-4_f64) * t48577 + F::cast_from(0.25410001404642664112e-5_f64) * t74304 - F::cast_from(0.17149607247227894789e-2_f64) * t5671 * t3936 * t22046 * t46951 + F::cast_from(0.42874018118069736972e-3_f64) * t5671 * t5673 * t22079 * t9840 + F::cast_from(0.85748036236139473944e-3_f64) * t5671 * t5673 * t5674 * t74314 + F::cast_from(0.20007875121765877254e-2_f64) * t74319 + F::cast_from(0.20007875121765877254e-2_f64) * t48591 - F::cast_from(0.27104001498285508387e-3_f64) * t74322 + F::new(5.0) / F::new(4.0) * t46730 * t800 * t6849 * t3829 + F::cast_from(0.16006300097412701803e-1_f64) * t48593;
    (t74314, t74329)
}
