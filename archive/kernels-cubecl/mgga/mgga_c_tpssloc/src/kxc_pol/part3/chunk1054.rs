//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1054/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1054<F: Float>(t13602: F, t12606: F, t883: F, t882: F, t123: F) -> (F, F, F) {
    let t13603 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13602;
    let t13611 = t883 * t12606;
    let t13612 = t882 * t13611;
    let t13613 = t123 * t13612;
    (t13603, t13611, t13613)
}
