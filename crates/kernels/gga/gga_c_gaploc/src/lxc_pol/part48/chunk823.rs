//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 823/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk823<F: Float>(t33232: F, t787: F, t9824: F, t41408: F, t10012: F, t2684: F, t2925: F, t9438: F, t3005: F, t9800: F, t9829: F, t13142: F, t7416: F) -> (F, F, F, F, F) {
    let t43991 = t787 * t33232 * t9824;
    let t43994 = F::cast_from(0.19171462976960374838e0_f64) * t41408;
    let t44001 = t2684 * t9438 * t10012 * t2925;
    let t44004 = t9800 * t3005 * t9829;
    let t44009 = t7416 * t13142;
    (t43991, t43994, t44001, t44004, t44009)
}
