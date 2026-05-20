//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3886/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3886<F: Float>(t13845: F, t13847: F, t13848: F, t21990: F, t13921: F, t22046: F, t22118: F, t3934: F, t4057: F, t49085: F, t49087: F, t49090: F, t49103: F, t5673: F, t5674: F, t74638: F, t74641: F, t74656: F, t74660: F, t9955: F) -> F {
    let t74664 = t13845 * t13847 * t13848 * t21990;
    let t74669 = -F::cast_from(0.2032800112371413129e-4_f64) * t74638 - F::cast_from(0.45178982497454656791e-5_f64) * t74641 - F::cast_from(0.16006300097412701803e-1_f64) * t49085 - F::cast_from(0.42874018118069736972e-3_f64) * t3934 * t5673 * t5674 * t13921 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5673 * t22046 * t4057 - F::cast_from(0.42874018118069736972e-2_f64) * t3934 * t9955 * t22118 * t4057 - F::cast_from(0.16006300097412701803e-1_f64) * t74656 - F::cast_from(0.2032800112371413129e-3_f64) * t74660 + F::cast_from(0.10164000561857065645e-3_f64) * t74664 - F::cast_from(0.36590402022685436322e-3_f64) * t49087 + F::cast_from(0.65057734796334705782e-3_f64) * t49090 + F::cast_from(0.36143185997963725432e-4_f64) * t49103;
    t74669
}
