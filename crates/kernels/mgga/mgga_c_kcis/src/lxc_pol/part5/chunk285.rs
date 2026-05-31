//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 285/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk285<F: Float>(t920: F, t924: F, t935: F, t1036: F, t245: F, t934: F) -> (F, F, F) {
    let t1040 = F::cast_from(0.41275e-2_f64) * t920;
    let t1042 = F::cast_from(0.1982e-1_f64) * t935 - t1040 - F::cast_from(0.41275e-2_f64) * t924;
    let t1045 = t1036 * t934 / F::cast_from(4.0_f64) + t245 * t1042 / F::cast_from(2.0_f64);
    (t1040, t1042, t1045)
}
