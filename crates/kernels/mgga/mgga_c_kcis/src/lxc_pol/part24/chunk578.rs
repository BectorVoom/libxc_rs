//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 578/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk578<F: Float>(t1291: F, t1872: F, t5044: F, t5049: F, t5051: F, t5054: F, t5056: F, t5058: F, t5060: F, t5063: F, t5065: F, t5069: F, t5071: F, t5074: F, t5079: F) -> (F, F) {
    let t5363 = t1872 * t1291;
    let t5379 = -F::new(0.9375e-1) * t5044 + F::new(0.1875e0) * t5049 - F::cast_from(0.13489583333333333333e-1_f64) * t5051 + F::new(0.25e0) * t5054 - F::new(0.25e0) * t5056 + F::new(0.625e-1) * t5058 + F::new(0.625e-1) * t5060 - F::new(0.625e-1) * t5063 - F::cast_from(0.13489583333333333333e-1_f64) * t5065 + F::cast_from(0.101171875e-1_f64) * t5069 - F::new(0.9375e-1) * t5071 + F::cast_from(0.13489583333333333333e-1_f64) * t5074 - F::cast_from(0.20833333333333333333e-1_f64) * t5079;
    (t5363, t5379)
}
