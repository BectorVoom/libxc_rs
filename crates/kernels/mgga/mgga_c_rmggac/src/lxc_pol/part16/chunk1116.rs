//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1116/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1116<F: Float>(t37558: F, t37560: F, t41365: F, t41367: F, t41368: F, t43628: F, t43629: F, t43631: F, t43632: F, t43633: F, t46300: F, t46302: F, t46305: F, t46307: F, t46309: F, t46311: F) -> F {
    let t49151 = F::cast_from(0.35481751119302649979e-2_f64) * t41365 - t41367 + t41368 + t37558 + t43628 + t43629 - t37560 - F::cast_from(0.79828278012425390427e-1_f64) * t46300 + F::cast_from(0.53218852008283593619e-1_f64) * t46302 - t43631 + t43632 + t43633 + F::cast_from(0.17701538806747441785e-2_f64) * t46305 - F::cast_from(0.21241846568096930142e-2_f64) * t46307 + F::cast_from(0.148692925976678511e-1_f64) * t46309 + F::cast_from(0.70806155226989767141e-2_f64) * t46311;
    t49151
}
