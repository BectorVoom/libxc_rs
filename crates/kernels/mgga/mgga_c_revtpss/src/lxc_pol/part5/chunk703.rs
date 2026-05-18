//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 703/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk703<F: Float>(t1610: F, t934: F, t2874: F, t1600: F, t2880: F, t918: F, t2848: F, t2884: F, t4571: F, t4576: F, t4581: F, t4585: F) -> (F, F, F, F, F) {
    let t4595 = t1610 * t934;
    let t4597 = F::new(2.0) * t2874 * t4595;
    let t4598 = t2880 * t1600;
    let t4599 = t4598 * t918;
    let t4606 = t2884 + t2848 / F::new(9.0) + t4571 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t4576 + F::new(2.0) / F::new(3.0) * t4581 - t4585 / F::new(3.0);
    (t4595, t4597, t4598, t4599, t4606)
}
