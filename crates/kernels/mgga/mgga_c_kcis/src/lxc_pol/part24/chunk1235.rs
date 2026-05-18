//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1235/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1235<F: Float>(t19745: F, t2842: F, t7718: F, t19807: F, t1262: F, t30045: F, t5329: F, t6737: F, t1851: F, t26996: F, t5341: F, t1267: F, t92735: F) -> (F, F, F, F, F) {
    let t100145 = t2842 * t7718 * t19745;
    let t100148 = t2842 * t7718 * t19807;
    let t100152 = t5329 * t30045 * t6737 * t1262;
    let t100157 = t5329 * t26996 * t1851 * t5341;
    let t100162 = t5329 * t92735 * t6737 * t1267;
    (t100145, t100148, t100152, t100157, t100162)
}
