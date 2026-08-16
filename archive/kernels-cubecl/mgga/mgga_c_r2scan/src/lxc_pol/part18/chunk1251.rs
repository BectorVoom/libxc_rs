//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1251/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1251<F: Float>(t3446: F, t3453: F, t9063: F, t37481: F, t37483: F, t40485: F, t42958: F, t42962: F, t42965: F, t42969: F, t42972: F, t42976: F, t43716: F, t43720: F, t43724: F, t43728: F, t43732: F) -> F {
    let t43887 = t3446 * t3453 * t9063;
    let t43889 = t42958 + t42962 - t42965 - t42969 + t37481 - t42972 - t42976 - t43716 - F::cast_from(0.1951603679568577289e-3_f64) * t37483 + t43720 - t43724 - t43728 + F::cast_from(0.29810146462873361018e-2_f64) * t40485 + t43732 - F::cast_from(0.36021158228745895953e-3_f64) * t43887;
    t43889
}
