//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 808/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk808<F: Float>(t1610: F, t934: F, t2874: F, t1600: F, t2880: F, t918: F, t2848: F, t2884: F, t4571: F, t4576: F, t4581: F, t4585: F) -> (F, F, F, F, F) {
    let t4595 = t1610 * t934;
    let t4597 = F::cast_from(2.0_f64) * t2874 * t4595;
    let t4598 = t2880 * t1600;
    let t4599 = t4598 * t918;
    let t4606 = t2884 + t2848 / F::cast_from(9.0_f64) + t4571 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4576 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4581 - t4585 / F::cast_from(3.0_f64);
    (t4595, t4597, t4598, t4599, t4606)
}
