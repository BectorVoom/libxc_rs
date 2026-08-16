//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1251/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1251(t14001: f64, t14463: f64, t3291: f64, t51214: f64, t14063: f64, t8962: f64, t51350: f64, t6684: f64, t3249: f64, t6238: f64, t899: f64, t923: f64) -> (f64, f64, f64, f64, f64) {
    let t53985 = t14001 * t14463;
    let t53986 = 7.0_f64 / 72.0_f64 * t53985;
    let t54014 = t51214 * t3291;
    let t54015 = 7.0_f64 / 576.0_f64 * t54014;
    let t54023 = t14063 * t8962;
    let t54047 = t6684 * t51350;
    let t54052 = t899 * t6238 * t923 * t3249;
    (t53986, t54015, t54023, t54047, t54052)
}
