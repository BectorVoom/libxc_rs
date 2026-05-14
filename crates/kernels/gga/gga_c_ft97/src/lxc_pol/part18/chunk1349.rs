//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1349/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1349<F: Float>(t105383: F, t105423: F, t105461: F, t105500: F, t105538: F, t105586: F, t105625: F, t105666: F, t105701: F, t105737: F, t105774: F, t105806: F, t105840: F, t105878: F, t105915: F, t105947: F, t605: F) -> (F,) {
    let t105952 = t605 * (t105383 + t105423 + t105461 + t105500 + t105538 + t105586 + t105625 + t105666 + t105701 + t105737 + t105774 + t105806 + t105840 + t105878 + t105915 + t105947);
    (t105952,)
}
