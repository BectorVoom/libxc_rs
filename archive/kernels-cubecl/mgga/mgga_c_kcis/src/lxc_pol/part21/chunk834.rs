//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 834/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk834<F: Float>(t3621: F, t426: F, t187: F, t2997: F, t1210: F, t3573: F, t396: F, t3576: F, t404: F, t962: F, t3031: F, t956: F) -> (F, F, F, F, F, F) {
    let t10819 = F::cast_from(1.0_f64) / t3621 / t426;
    let t10845 = t187 * t2997;
    let t10861 = F::cast_from(1.0_f64) / t3573 / t1210;
    let t10862 = t396 * t10861;
    let t10865 = F::cast_from(1.0_f64) / t3576 / t404;
    let t10869 = t2997 * t962;
    let t10874 = t956 * t3031;
    (t10819, t10845, t10862, t10865, t10869, t10874)
}
