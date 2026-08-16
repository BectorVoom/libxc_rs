//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1007/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1007<F: Float>(t1083: F, t33883: F, t1980: F, t7458: F, t1089: F, t15995: F, t2090: F, t598: F, t535: F, t7457: F, t7459: F, t3201: F, t8484: F) -> (F, F, F, F, F) {
    let t33884 = t1083 * t33883;
    let t33886 = t1980 * t7458 * t33884;
    let t33887 = F::cast_from(0.28582678745379824648e-3_f64) * t33886;
    let t33890 = t598 * t1089 * t15995 * t2090;
    let t33894 = t7457 * t7458 * t535 * t7459;
    let t33898 = t598 * t1089 * t3201 * t8484;
    (t33884, t33887, t33890, t33894, t33898)
}
