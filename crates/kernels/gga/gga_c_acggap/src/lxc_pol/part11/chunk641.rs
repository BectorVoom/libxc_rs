//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 641/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk641<F: Float>(t2959: F, t2961: F, t2963: F, t2966: F, t1381: F, t912: F, t2971: F, t2710: F, t2713: F, t2717: F, t2737: F, t2957: F, t2969: F, t4061: F, t4063: F, t4065: F, t4069: F) -> F {
    let t5022 = F::cast_from(0.5848223622634646207e0_f64) * t2959;
    let t5023 = F::cast_from(0.34631718211362927518e2_f64) * t2961;
    let t5024 = F::cast_from(0.4883052614935078681e-3_f64) * t2963;
    let t5025 = F::cast_from(0.18311447306006545054e-3_f64) * t2966;
    let t5026 = t1381 * t912;
    let t5027 = F::cast_from(0.11696447245269292414e1_f64) * t5026;
    let t5028 = F::new(48.0) * t2971;
    let t5029 = t4061 - t4063 + t4065 + t4069 - t2957 - t5022 - t5023 + t5024 - t5025 + t2710 - t2713 - t2717 + t2737 + t5027 - t2969 + t5028;
    t5029
}
