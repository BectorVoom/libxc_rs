//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 760/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk760<F: Float>(t7890: F, t8117: F, t944: F, t2146: F, t2236: F, t2241: F, t7912: F, t8062: F, t8067: F, t8069: F, t8074: F, t8076: F, t8078: F, t8082: F, t8087: F, t8092: F, t8096: F, t8098: F, t8101: F, t8106: F, t8108: F, t8113: F, t8115: F) -> (F, F) {
    let t8119 = t7890 * t8117 * t944;
    let t8122 = F::cast_from(0.17347256376410398924e1_f64) * t7912 * t2236 + F::cast_from(0.17347256376410398924e1_f64) * t8062 + t8067 + F::cast_from(0.17347256376410398924e1_f64) * t2146 * t8069 + F::cast_from(0.34694512752820797848e1_f64) * t8074 + F::cast_from(0.17347256376410398924e1_f64) * t8076 - F::cast_from(0.17347256376410398924e1_f64) * t8078 - F::cast_from(0.34694512752820797848e1_f64) * t8082 + t8087 + F::cast_from(0.8673628188205199462e0_f64) * t7912 * t2241 + F::cast_from(0.4336814094102599731e0_f64) * t2146 * t8092 - t8096 - t8098 - F::cast_from(0.17347256376410398924e1_f64) * t8101 - t8106 - F::cast_from(0.26020884564615598386e1_f64) * t2146 * t8108 - t8113 - F::cast_from(0.13170898365871023197e1_f64) * t8115 - F::cast_from(0.8673628188205199462e0_f64) * t2146 * t8119;
    (t8119, t8122)
}
