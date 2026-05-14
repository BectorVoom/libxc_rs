//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 941/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk941<F: Float>(t153: F, t156: F, t18054: F, t18367: F, t18369: F, t18372: F, t18375: F, t18377: F, t18379: F, t18413: F, t18415: F, t18416: F, t18419: F, t18420: F, t18987: F, t242: F) -> (F,) {
    let t18991 = 0.10051538464260528225e1 * t18367 + 0.10051538464260528225e1 * t18369 + t18372 - 0.83762820535504401876e-1 * t18054 * t242 - 0.33505128214201760751e0 * t18375 - 0.50257692321302641126e0 * t18377 - 0.33505128214201760751e0 * t18379 - t18413 + t18415 - 0.10051538464260528225e1 * t18416 - t18419 + 0.2010307692852105645e1 * t18420 + 0.42708890021612718669e0 * t153 * t156 * t18987;
    (t18991,)
}
