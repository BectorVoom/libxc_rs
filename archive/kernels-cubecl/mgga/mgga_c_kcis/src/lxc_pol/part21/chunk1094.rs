//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1094/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1094<F: Float>(t1021: F, t3358: F, t3355: F, t3348: F, t26889: F, t26892: F, t26894: F, t26898: F, t26900: F, t26902: F, t26904: F, t26906: F, t26908: F) -> (F, F, F, F) {
    let t26910 = t1021 * t3358;
    let t26912 = t1021 * t3355;
    let t26914 = t1021 * t3348;
    let t26916 = -t26889 / F::cast_from(64.0_f64) + t26892 / F::cast_from(3.0_f64) - t26894 / F::cast_from(12.0_f64) + t26898 / F::cast_from(8.0_f64) - t26900 / F::cast_from(96.0_f64) + t26902 / F::cast_from(128.0_f64) + t26904 / F::cast_from(12.0_f64) - t26906 / F::cast_from(48.0_f64) + t26908 / F::cast_from(64.0_f64) + t26910 / F::cast_from(9.0_f64) - F::cast_from(19.0_f64) / F::cast_from(72.0_f64) * t26912 - t26914 / F::cast_from(288.0_f64);
    (t26910, t26912, t26914, t26916)
}
