//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1200/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1200<F: Float>(t32298: F, t7898: F, t33976: F, t7235: F, t119578: F, t28067: F, t28167: F, t37972: F, t5627: F, t28177: F, t8568: F, t34258: F, t7003: F) -> (F, F, F, F, F, F) {
    let t127332 = t7898 * t32298;
    let t127335 = t7235 * t33976;
    let t127336 = t119578 * t28067;
    let t127340 = F::cast_from(6.0_f64) * t28167 * t37972 * t5627;
    let t127341 = t8568 * t28177;
    let t127346 = F::cast_from(4.0_f64) * t34258 * t7003;
    (t127332, t127335, t127336, t127340, t127341, t127346)
}
