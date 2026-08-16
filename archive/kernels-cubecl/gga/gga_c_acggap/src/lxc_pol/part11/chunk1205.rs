//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1205/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1205<F: Float>(t1659: F, t20138: F, t2127: F, t2155: F, t2338: F, t32196: F, t32201: F, t32210: F, t32219: F, t33566: F, t35324: F, t36433: F, t36498: F, t36504: F, t36511: F, t36515: F, t36526: F, t5340: F, t7879: F, t7931: F, t7932: F, t7934: F, t8001: F, t8400: F, t9033: F) -> F {
    let t36528 = -F::cast_from(0.17347256376410398924e1_f64) * t8400 * t9033 * t20138 - t36498 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t36433 * t7934 + F::cast_from(0.17347256376410398924e1_f64) * t32196 + F::cast_from(0.8673628188205199462e0_f64) * t32201 - F::cast_from(0.65854491829355115987e0_f64) * t36504 - F::cast_from(0.4336814094102599731e0_f64) * t2338 * t7879 + F::cast_from(0.8673628188205199462e0_f64) * t33566 * t2155 - t32210 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t7932 * t36511 + F::cast_from(0.26020884564615598386e1_f64) * t8400 * t36515 * t35324 + F::cast_from(0.13170898365871023197e1_f64) * t2127 * t5340 + F::cast_from(0.17347256376410398924e1_f64) * t32219 - F::cast_from(0.13170898365871023197e1_f64) * t8001 * t1659 - F::cast_from(0.26020884564615598386e1_f64) * t36526;
    t36528
}
