//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1195/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1195<F: Float>(t232: F, t24677: F, t24690: F, t7680: F, t774: F, t7682: F, t216: F, t2371: F, t2414: F, t24303: F, t7672: F, t7629: F, t7689: F) -> (F, F, F, F) {
    let t24693 = F::cast_from(0.62182e-1_f64) * (t24677 + t24690) * t232;
    let t24694 = t774 * t7680;
    let t24696 = F::cast_from(0.38596378373162651572e3_f64) * t24694 * t7682;
    let t24699 = t216 / t2414 / t2371;
    let t24702 = F::cast_from(0.620700176468474021e4_f64) * t24699 * t24303 * t7672;
    let t24704 = F::cast_from(24.0_f64) * t7629 * t7689;
    (t24693, t24696, t24702, t24704)
}
