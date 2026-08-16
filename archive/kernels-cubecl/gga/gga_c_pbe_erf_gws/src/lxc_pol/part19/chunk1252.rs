//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1252/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1252<F: Float>(t14311: F, t3083: F, t4083: F, t8669: F, t4110: F, t8589: F, t829: F, t830: F, t52991: F, t53011: F, t14182: F, t26958: F) -> (F, F, F, F, F, F) {
    let t54911 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t3083 * t14311;
    let t54915 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8669 * t4083;
    let t54916 = t8589 * t4110;
    let t54918 = t829 * t830 * t54916;
    let t54923 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t52991;
    let t54927 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t53011;
    let t54937 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t26958 * t14182;
    (t54911, t54915, t54918, t54923, t54927, t54937)
}
