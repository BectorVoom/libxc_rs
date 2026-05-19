//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1101/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1101<F: Float>(t35096: F, t1181: F, t21118: F, t7351: F, t7426: F, t1131: F, t525: F, t2068: F, t604: F, t33706: F, t599: F, t1165: F, t21955: F, t30806: F) -> (F, F, F, F, F, F) {
    let t35097 = F::cast_from(0.21437009059034868486e-2_f64) * t35096;
    let t35100 = t7426 * t1181 * t7351 * t21118;
    let t35101 = F::cast_from(0.12862205435420921092e-2_f64) * t35100;
    let t35102 = t525 * t1131;
    let t35105 = t2068 * t1181 * t604 * t35102;
    let t35109 = t2068 * t1181 * t599 * t33706;
    let t35113 = t30806 * t1165 * t604 * t21955;
    (t35097, t35101, t35102, t35105, t35109, t35113)
}
