//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 983/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk983<F: Float>(t7306: F, t8397: F, t2331: F, t394: F, t1960: F, t5379: F, t7980: F, t2132: F, t2138: F, t322: F, t8993: F, t2147: F, t2341: F, t7885: F, t864: F) -> (F, F, F, F, F, F) {
    let t33488 = F::cast_from(0.34694512752820797848e1_f64) * t8397 * t7306;
    let t33489 = t394 * t2331;
    let t33496 = F::cast_from(0.13170898365871023197e1_f64) * t1960 * t5379;
    let t33500 = F::cast_from(0.17347256376410398924e1_f64) * t8397 * t7980;
    let t33504 = F::cast_from(0.17347256376410398924e1_f64) * t2138 * t2132 * t8993 * t322;
    let t33507 = t7885 * t2147 * t2341 * t864;
    (t33488, t33489, t33496, t33500, t33504, t33507)
}
