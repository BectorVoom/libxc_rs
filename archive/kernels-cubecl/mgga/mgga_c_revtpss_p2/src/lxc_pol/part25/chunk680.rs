//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 680/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk680<F: Float>(t1370: F, t1388: F, t3926: F, t3931: F, t3934: F, t3940: F, t3944: F, t3946: F, t3950: F, t3953: F, t3956: F, t3958: F, t3961: F, t3967: F, t4065: F) -> F {
    let t4066 = -F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t3926 + F::cast_from(0.20007875121765877254e-2_f64) * t3931 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t3940 + t3944 * t3946 / F::cast_from(16.0_f64) + t3950 + F::cast_from(0.57165357490759649296e-4_f64) * t3953 + t3956 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t3958 - t1370 * t3961 / F::cast_from(48.0_f64) + t3967 + t4065;
    t4066
}
