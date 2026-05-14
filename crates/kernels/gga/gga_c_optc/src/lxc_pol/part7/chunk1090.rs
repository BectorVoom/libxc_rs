//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1090/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1090<F: Float>(t24694: F, t7682: F, t216: F, t2371: F, t2414: F, t24303: F, t7672: F, t7629: F, t7689: F, t24228: F, t24230: F, t24233: F, t24299: F, t24308: F, t24337: F, t24339: F, t24344: F, t24693: F) -> (F, F, F, F) {
    let t24696 = 0.38596378373162651572e3 * t24694 * t7682;
    let t24699 = t216 / t2414 / t2371;
    let t24702 = 0.620700176468474021e4 * t24699 * t24303 * t7672;
    let t24704 = 24.0 * t7629 * t7689;
    let t24705 = t24228 + t24230 + t24233 + t24299 + t24308 + t24337 + t24339 - t24344 - t24693 - t24696 - t24702 + t24704;
    (t24696, t24702, t24704, t24705)
}
