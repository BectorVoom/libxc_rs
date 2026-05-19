//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 988/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk988<F: Float>(t1264: F, t525: F, t33428: F, t615: F, t8396: F, t862: F, t7898: F, t315: F, t323: F, t8993: F, t2149: F, t29997: F, t31912: F, t31916: F, t31918: F, t31922: F, t31926: F, t31928: F, t31937: F, t7893: F, t7912: F, t7931: F, t7932: F, t8400: F, t8402: F, t8415: F, t9003: F) -> (F, F) {
    let t33561 = t525 * t1264;
    let t33566 = t615 * t33428;
    let t33574 = t862 * t8396;
    let t33575 = t33574 * t7898;
    let t33586 = F::cast_from(0.13170898365871023197e1_f64) * t315 * t8993 * t323;
    let t33588 = -F::cast_from(0.8673628188205199462e0_f64) * t7931 * t7932 * t33561 - F::cast_from(0.17347256376410398924e1_f64) * t31912 + F::cast_from(0.17347256376410398924e1_f64) * t33566 * t2149 + F::cast_from(0.17347256376410398924e1_f64) * t7912 * t8415 - F::cast_from(0.13170898365871023197e1_f64) * t31916 - F::cast_from(0.65854491829355115987e0_f64) * t31918 - F::cast_from(0.34694512752820797848e1_f64) * t31922 + F::cast_from(0.34694512752820797848e1_f64) * t33575 + F::cast_from(0.17347256376410398924e1_f64) * t31926 - F::cast_from(0.8673628188205199462e0_f64) * t9003 * t7893 + F::cast_from(0.13170898365871023197e1_f64) * t31928 + F::cast_from(0.8673628188205199462e0_f64) * t8400 * t29997 * t8402 - t33586 - F::cast_from(0.17347256376410398924e1_f64) * t31937;
    (t33566, t33588)
}
