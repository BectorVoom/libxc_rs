//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1324;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta304<F: Float>(t10383: F, t339: F, t3069: F, t3180: F, t3036: F, t67: F, t3067: F, t3186: F, t3062: F, t820: F, t3200: F, t3051: F, t1005: F, t3082: F, t121: F, t3061: F, t1008: F, t349: F, t1011: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10385, t10390, t10401, t10403, t10408, t10413, t10422) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1324::<F>(t10383, t339, t3069, t3180, t3036, t67, t3067, t3186, t3062, t820, t3200, t3051);
        let (t10436, t10457, t10469, t10470, t10471) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1325::<F>(t1005, t3082, t121, t3061, t1008, t349, t1011);
    (t10385, t10390, t10401, t10403, t10408, t10413, t10422, t10436, t10457, t10469, t10470, t10471)
}
