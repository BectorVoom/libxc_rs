//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1357/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1357<F: Float>(t66390: F, t66393: F, t66394: F, t66399: F, t69926: F, t69928: F, t69930: F, t69932: F, t69934: F, t69936: F, t69938: F, t69941: F) -> F {
    let t72044 = -F::cast_from(5.0_f64) / F::cast_from(96.0_f64) * t69926 + t69928 / F::cast_from(96.0_f64) - t69930 / F::cast_from(48.0_f64) - t66390 + t69932 / F::cast_from(192.0_f64) + t69934 / F::cast_from(192.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t69936 + t69938 / F::cast_from(384.0_f64) - t66393 - t66394 + t66399 + t69941 / F::cast_from(8.0_f64);
    t72044
}
