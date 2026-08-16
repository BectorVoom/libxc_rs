//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1006/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1006<F: Float>(t1480: F, t1483: F, t2290: F, t44: F, t56: F, t5835: F, t5838: F, t5843: F, t5848: F, t5851: F, t61: F, t38: F) -> (F, F) {
    let t5854 = F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t44 * t5835 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t5838 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t5843 * t61 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t1480 * t1483 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t56 * t5848 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t5851 - t2290;
    let t5855 = t38 * t5854;
    (t5854, t5855)
}
