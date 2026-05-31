//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1335/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1335<F: Float>(t54135: F, t54152: F, t51252: F, t54133: F, t54137: F, t54139: F, t54142: F, t54144: F, t54146: F, t54148: F, t54150: F, t54154: F) -> F {
    let t55491 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54135;
    let t55500 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t54152;
    let t55502 = t54133 / F::cast_from(8.0_f64) - t55491 + t54137 / F::cast_from(128.0_f64) + F::cast_from(3.0_f64) / F::cast_from(128.0_f64) * t54139 - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t51252 + t54142 / F::cast_from(48.0_f64) - t54144 / F::cast_from(192.0_f64) - t54146 / F::cast_from(48.0_f64) + t54148 / F::cast_from(24.0_f64) - t54150 / F::cast_from(48.0_f64) + t55500 - t54154 / F::cast_from(192.0_f64);
    t55502
}
