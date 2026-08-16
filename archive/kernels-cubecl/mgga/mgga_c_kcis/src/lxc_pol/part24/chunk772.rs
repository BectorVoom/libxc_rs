//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 772/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk772<F: Float>(t10752: F, t1095: F, t1169: F, t983: F, t381: F, t9531: F, t3621: F, t426: F, t1210: F, t3573: F, t396: F, t3576: F, t404: F) -> (F, F, F, F, F, F) {
    let t10753 = t1095 * t10752;
    let t10787 = t1169 * t983;
    let t10796 = t9531 * t381;
    let t10819 = F::cast_from(1.0_f64) / t3621 / t426;
    let t10861 = F::cast_from(1.0_f64) / t3573 / t1210;
    let t10862 = t396 * t10861;
    let t10865 = F::cast_from(1.0_f64) / t3576 / t404;
    (t10753, t10787, t10796, t10819, t10862, t10865)
}
