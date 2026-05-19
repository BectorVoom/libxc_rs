//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 979/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk979<F: Float>(t32092: F, t9030: F, t30029: F, t8407: F, t1603: F, t618: F, t2137: F, t2140: F, t1614: F, t7976: F, t29988: F, t557: F) -> (F, F, F, F, F, F) {
    let t33414 = F::cast_from(0.17347256376410398924e1_f64) * t32092 * t9030;
    let t33416 = F::cast_from(0.17347256376410398924e1_f64) * t30029 * t8407;
    let t33428 = t1603 * t618;
    let t33429 = t2137 * t33428;
    let t33431 = F::cast_from(0.17347256376410398924e1_f64) * t33429 * t2140;
    let t33435 = F::cast_from(0.13170898365871023197e1_f64) * t7976 * t1614;
    let t33437 = F::cast_from(0.13170898365871023197e1_f64) * t29988 * t557;
    (t33414, t33416, t33428, t33431, t33435, t33437)
}
