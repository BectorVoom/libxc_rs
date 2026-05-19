//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1202/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1202<F: Float>(t2138: F, t2147: F, t322: F, t8392: F, t7998: F, t8397: F, t1658: F, t2122: F, t2146: F, t31965: F, t32124: F, t32133: F, t32135: F, t32143: F, t32150: F, t32157: F, t33535: F, t36432: F, t36436: F, t36439: F, t36447: F, t7934: F, t9026: F) -> F {
    let t36452 = F::cast_from(0.34694512752820797848e1_f64) * t2138 * t2147 * t8392 * t322;
    let t36460 = t8397 * t7998;
    let t36463 = -t36432 - t36436 + t36439 - F::cast_from(0.34694512752820797848e1_f64) * t32133 + F::cast_from(0.52041769129231196772e1_f64) * t32124 * t33535 * t7934 - t36447 + F::cast_from(0.13170898365871023197e1_f64) * t32135 - t36452 + F::cast_from(0.69389025505641595696e1_f64) * t32143 - F::cast_from(0.17347256376410398924e1_f64) * t31965 * t9026 + t32150 + F::cast_from(0.17347256376410398924e1_f64) * t2146 * t2147 * t2122 * t1658 - F::cast_from(0.8673628188205199462e0_f64) * t36460 + F::cast_from(0.17347256376410398924e1_f64) * t32157;
    t36463
}
