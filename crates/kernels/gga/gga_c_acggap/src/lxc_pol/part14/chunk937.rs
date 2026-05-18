//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 937/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk937<F: Float>(t2140: F, t33429: F, t1614: F, t7976: F, t29988: F, t557: F, t2132: F, t2138: F, t2331: F, t879: F, t2147: F, t2341: F) -> (F, F, F, F, F) {
    let t33431 = F::new(0.17347256376410398924e1) * t33429 * t2140;
    let t33435 = F::new(0.13170898365871023197e1) * t7976 * t1614;
    let t33437 = F::new(0.13170898365871023197e1) * t29988 * t557;
    let t33444 = t2138 * t2132 * t2331 * t879;
    let t33451 = t2138 * t2147 * t2341 * t879;
    (t33431, t33435, t33437, t33444, t33451)
}
