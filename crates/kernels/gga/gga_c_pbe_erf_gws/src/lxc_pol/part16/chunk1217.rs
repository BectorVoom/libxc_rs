//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1217/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1217<F: Float>(t51977: F, t2242: F, t4113: F, t1208: F, t6729: F, t1206: F, t2100: F, t353: F, t859: F, t14182: F, t19906: F, t4083: F, t4474: F) -> (F, F, F, F, F, F) {
    let t52582 = F::new(455.0) / F::new(648.0) * t51977;
    let t52586 = t2242 * t4113;
    let t52589 = F::new(455.0) / F::new(1296.0) * t6729 * t1208;
    let t52600 = t859 * t353 * t1206 * t2100;
    let t52603 = t19906 * t14182;
    let t52607 = t4474 * t4083;
    (t52582, t52586, t52589, t52600, t52603, t52607)
}
