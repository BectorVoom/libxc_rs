//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 663/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk663<F: Float>(t43: F, t50: F, t3629: F, t3631: F, t3633: F, t3635: F, zeta_threshold: F) -> F {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t3711 = piecewise3::<F>(t44, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3629 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3631);
    let t3715 = piecewise3::<F>(t51, F::cast_from(0.0_f64), -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3633 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3635);
    let t3717 = t3711 / F::cast_from(2.0_f64) + t3715 / F::cast_from(2.0_f64);
    t3717
}
