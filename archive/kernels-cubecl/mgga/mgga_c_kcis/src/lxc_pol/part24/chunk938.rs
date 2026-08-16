//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 938/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk938<F: Float>(t19895: F, t19897: F, t19899: F, t19902: F, t19906: F, t19909: F, t19912: F, t19914: F, t19916: F, t19918: F, t19920: F, t19922: F, t19924: F, t19927: F, t19929: F, t19932: F, t19935: F, t19938: F, t19941: F) -> F {
    let t19943 = -t19895 / F::cast_from(576.0_f64) - t19897 / F::cast_from(18.0_f64) - t19899 / F::cast_from(8.0_f64) - t19902 / F::cast_from(64.0_f64) - t19906 / F::cast_from(256.0_f64) - t19909 / F::cast_from(576.0_f64) + t19912 / F::cast_from(108.0_f64) + t19914 / F::cast_from(256.0_f64) + t19916 / F::cast_from(128.0_f64) + t19918 / F::cast_from(12.0_f64) - t19920 / F::cast_from(12.0_f64) - t19922 / F::cast_from(128.0_f64) - t19924 / F::cast_from(16.0_f64) - t19927 / F::cast_from(12.0_f64) - t19929 / F::cast_from(24.0_f64) - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t19932 + t19935 / F::cast_from(72.0_f64) + t19938 / F::cast_from(54.0_f64) - t19941 / F::cast_from(96.0_f64);
    t19943
}
