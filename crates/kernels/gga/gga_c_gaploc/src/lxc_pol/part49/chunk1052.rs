//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1052/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1052<F: Float>(t3025: F, t3255: F, t4752: F, t33232: F, t787: F, t9824: F, t41405: F, t41408: F, t43586: F, t7584: F, t7585: F, t10012: F, t2684: F, t2925: F, t9438: F) -> (F, F, F, F, F, F) {
    let t43989 = F::cast_from(0.7150097990370085334e0_f64) * t3025 * t4752 * t3255;
    let t43991 = t787 * t33232 * t9824;
    let t43993 = F::cast_from(0.20854452471912748891e0_f64) * t41405;
    let t43994 = F::cast_from(0.19171462976960374838e0_f64) * t41408;
    let t43997 = t7584 * t7585 * t43586;
    let t44001 = t2684 * t9438 * t10012 * t2925;
    (t43989, t43991, t43993, t43994, t43997, t44001)
}
