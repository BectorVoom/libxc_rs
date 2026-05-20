//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1478/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1478<F: Float>(t300: F, t41778: F, t41825: F, t41853: F, t41930: F, t3333: F, t3335: F, t11598: F, t3022: F, t198: F, t336: F, t41577: F, t41580: F, t41582: F, t41585: F, t41591: F, t41657: F, t41841: F, t41845: F, t41847: F, t41849: F) -> (F, F, F) {
    let t41933 = t300 * (t41778 + t41825 + t41853 + t41930);
    let t41934 = t3333 * t3333;
    let t41936 = t3335 * t3335;
    let t41937 = F::new(1.0) / t41936;
    let t41942 = F::cast_from(0.14035736694323150897e2_f64) * t3022 * t11598;
    let t41943 = -F::new(6.0) * t198 * t336 * t41934 * t41937 + t41577 + t41580 + t41582 + t41585 - t41591 + t41657 + t41841 + t41845 - t41847 + t41849 + t41933 - t41942;
    (t41933, t41942, t41943)
}
