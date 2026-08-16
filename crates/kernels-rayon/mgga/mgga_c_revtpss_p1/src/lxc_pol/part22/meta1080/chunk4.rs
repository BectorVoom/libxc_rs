//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3886/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3886(t13845: f64, t13847: f64, t13848: f64, t21990: f64, t13921: f64, t22046: f64, t22118: f64, t3934: f64, t4057: f64, t49085: f64, t49087: f64, t49090: f64, t49103: f64, t5673: f64, t5674: f64, t74638: f64, t74641: f64, t74656: f64, t74660: f64, t9955: f64) -> f64 {
    let t74664 = t13845 * t13847 * t13848 * t21990;
    let t74669 = -0.2032800112371413129e-4_f64 * t74638 - 0.45178982497454656791e-5_f64 * t74641 - 0.16006300097412701803e-1_f64 * t49085 - 0.42874018118069736972e-3_f64 * t3934 * t5673 * t5674 * t13921 - 0.21437009059034868486e-3_f64 * t3934 * t5673 * t22046 * t4057 - 0.42874018118069736972e-2_f64 * t3934 * t9955 * t22118 * t4057 - 0.16006300097412701803e-1_f64 * t74656 - 0.2032800112371413129e-3_f64 * t74660 + 0.10164000561857065645e-3_f64 * t74664 - 0.36590402022685436322e-3_f64 * t49087 + 0.65057734796334705782e-3_f64 * t49090 + 0.36143185997963725432e-4_f64 * t49103;
    t74669
}
