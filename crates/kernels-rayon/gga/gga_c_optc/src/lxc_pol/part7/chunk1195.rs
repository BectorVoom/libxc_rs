//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1195/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1195(t232: f64, t24677: f64, t24690: f64, t7680: f64, t774: f64, t7682: f64, t216: f64, t2371: f64, t2414: f64, t24303: f64, t7672: f64, t7629: f64, t7689: f64) -> (f64, f64, f64, f64) {
    let t24693 = 0.62182e-1_f64 * (t24677 + t24690) * t232;
    let t24694 = t774 * t7680;
    let t24696 = 0.38596378373162651572e3_f64 * t24694 * t7682;
    let t24699 = t216 / t2414 / t2371;
    let t24702 = 0.620700176468474021e4_f64 * t24699 * t24303 * t7672;
    let t24704 = 24.0_f64 * t7629 * t7689;
    (t24693, t24696, t24702, t24704)
}
