//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1251/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1251<F: Float>(t14001: F, t14463: F, t3291: F, t51214: F, t14063: F, t8962: F, t51350: F, t6684: F, t3249: F, t6238: F, t899: F, t923: F) -> (F, F, F, F, F) {
    let t53985 = t14001 * t14463;
    let t53986 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t53985;
    let t54014 = t51214 * t3291;
    let t54015 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t54014;
    let t54023 = t14063 * t8962;
    let t54047 = t6684 * t51350;
    let t54052 = t899 * t6238 * t923 * t3249;
    (t53986, t54015, t54023, t54047, t54052)
}
