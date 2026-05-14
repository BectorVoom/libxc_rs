//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1093/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1093<F: Float>(t32782: F, t32795: F, t32796: F, t32799: F, t32800: F, t32803: F, t35560: F, t35562: F, t35563: F, t37605: F, t37606: F, t37607: F, t40043: F, t40045: F, t40047: F, t40050: F, t40054: F, t40057: F) -> (F,) {
    let t41911 = t32782 - t37605 + t37606 - 0.12579236915841660828e-2 * t40043 - t37607 + 35.0 / 108.0 * t35560 - t32795 - t32796 - 0.84046875e-1 * t40045 - 0.5603125e-1 * t40047 + t40050 / 4.0 + t35562 + t35563 + t32799 - t32800 - t32803 - 0.68598428988911579155e-1 * t40054 + 0.12862205435420921092e-1 * t40057;
    (t41911,)
}
