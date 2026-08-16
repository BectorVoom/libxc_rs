//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1101/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1101<F: Float>(t10848: F, t3415: F, t40498: F, t40527: F, t40547: F, t40563: F, t47586: F, t47587: F, t47616: F, t47617: F, t47618: F, t47622: F, t47626: F) -> (F, F, F, F, F, F) {
    let t47628 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t10848 * t3415;
    let t47629 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t40498;
    let t47630 = F::cast_from(64.0_f64) / F::cast_from(45.0_f64) * t40527;
    let t47631 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t40547;
    let t47632 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t40563;
    let t47633 = t47586 - t47587 + t47616 - t47617 - t47618 + t47622 - t47626 - t47628 + t47629 + t47630 - t47631 - t47632;
    (t47628, t47629, t47630, t47631, t47632, t47633)
}
