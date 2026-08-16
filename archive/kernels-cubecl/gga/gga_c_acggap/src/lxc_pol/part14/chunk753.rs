//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 753/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk753<F: Float>(t463: F, t525: F, t7932: F, t2146: F, t7307: F, t7889: F, t7931: F, t8393: F, t8398: F, t8400: F, t8403: F, t8408: F, t8411: F, t8415: F, t8420: F, t8424: F, t8428: F, t8433: F, t8437: F) -> (F, F, F) {
    let t8440 = t525 * t463;
    let t8441 = t7932 * t8440;
    let t8444 = F::cast_from(0.17347256376410398924e1_f64) * t7307 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t8393 - F::cast_from(0.8673628188205199462e0_f64) * t8398 + F::cast_from(0.4336814094102599731e0_f64) * t8400 * t8403 - F::cast_from(0.8673628188205199462e0_f64) * t8408 - t7889 - F::cast_from(0.26020884564615598386e1_f64) * t2146 * t8411 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t8415 + F::cast_from(0.8673628188205199462e0_f64) * t8420 - F::cast_from(0.8673628188205199462e0_f64) * t8424 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t8428 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t8433 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t8437 - F::cast_from(0.8673628188205199462e0_f64) * t7931 * t8441;
    (t8440, t8441, t8444)
}
