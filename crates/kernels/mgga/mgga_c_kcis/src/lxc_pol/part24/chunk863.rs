//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 863/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk863<F: Float>(t2933: F, t6390: F, t6380: F, t659: F, t18681: F, t945: F, t26: F, t6320: F, t9752: F, t934: F, t4625: F, t4700: F) -> (F, F, F, F, F) {
    let t18874 = F::cast_from(1.0_f64) * t2933 * t6390;
    let t18877 = t659 * t6380;
    let t18879 = t945 * t18681;
    let t18880 = t26 * t18879;
    let t18884 = t9752 * t6320;
    let t18885 = t18884 * t934;
    let t18887 = t4700 * t4625;
    (t18874, t18877, t18880, t18885, t18887)
}
