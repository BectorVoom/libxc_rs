//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1007/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1007<F: Float>(t35573: F, t1454: F, t30148: F, t30159: F, t7586: F, t1460: F, t355: F, t3706: F, t7842: F, t30374: F, t8606: F, t7426: F, t7569: F, t8480: F) -> (F, F, F, F, F) {
    let t35574 = F::new(0.31448092289604152068e-2) * t35573;
    let t35580 = t30159 * t7586 * t30148 * t1454;
    let t35581 = F::new(0.12579236915841660827e-2) * t35580;
    let t35585 = t30159 * t7842 * t3706 * t355 * t1460;
    let t35586 = F::new(0.25158473831683321654e-2) * t35585;
    let t35587 = t30374 * t8606;
    let t35594 = t7426 * t8480 * t7569;
    (t35574, t35581, t35586, t35587, t35594)
}
