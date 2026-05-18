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
    let t26916 = -t26889 / F::new(64.0) + t26892 / F::new(3.0) - t26894 / F::new(12.0) + t26898 / F::new(8.0) - t26900 / F::new(96.0) + t26902 / F::new(128.0) + t26904 / F::new(12.0) - t26906 / F::new(48.0) + t26908 / F::new(64.0) + t26910 / F::new(9.0) - F::new(19.0) / F::new(72.0) * t26912 - t26914 / F::new(288.0);
    (t26910, t26912, t26914, t26916)
}
