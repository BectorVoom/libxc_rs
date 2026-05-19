//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 981/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk981<F: Float>(t2132: F, t2138: F, t2331: F, t879: F, t2147: F, t2341: F, t872: F, t9062: F, t2347: F, t30005: F, t7990: F, t8419: F) -> (F, F, F, F, F) {
    let t33444 = t2138 * t2132 * t2331 * t879;
    let t33451 = t2138 * t2147 * t2341 * t879;
    let t33459 = F::cast_from(0.13170898365871023197e1_f64) * t9062 * t872;
    let t33465 = t30005 * t2347;
    let t33468 = F::cast_from(0.17347256376410398924e1_f64) * t7990 * t8419;
    (t33444, t33451, t33459, t33465, t33468)
}
