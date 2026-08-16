//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 815/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk815<F: Float>(t2152: F, t9502: F, t157: F, t524: F, t556: F, t7932: F, t2146: F, t2338: F, t2347: F, t2351: F, t7889: F, t7900: F, t7921: F, t7926: F, t7929: F, t7931: F, t8398: F, t8408: F, t8420: F, t8424: F, t8999: F, t9003: F, t9008: F, t9018: F, t9031: F, t9493: F, t9498: F) -> (F, F, F, F) {
    let t9503 = t2152 * t9502;
    let t9508 = t556 * t524 * t157;
    let t9509 = t7932 * t9508;
    let t9514 = -F::cast_from(0.17347256376410398924e1_f64) * t8398 + F::cast_from(0.8673628188205199462e0_f64) * t9003 * t2347 - F::cast_from(0.17347256376410398924e1_f64) * t8408 - t7889 + F::cast_from(0.17347256376410398924e1_f64) * t8420 - F::cast_from(0.17347256376410398924e1_f64) * t8424 + F::cast_from(0.17347256376410398924e1_f64) * t8999 - F::cast_from(0.8673628188205199462e0_f64) * t2146 * t9493 + t7900 + F::cast_from(0.13170898365871023197e1_f64) * t9008 - F::cast_from(0.26020884564615598386e1_f64) * t2146 * t9498 - t7921 + F::cast_from(0.13170898365871023197e1_f64) * t9018 - t7926 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t9503 + F::cast_from(0.17347256376410398924e1_f64) * t9031 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t9509 - F::cast_from(0.8673628188205199462e0_f64) * t2338 * t2351 - t7929;
    (t9503, t9508, t9509, t9514)
}
