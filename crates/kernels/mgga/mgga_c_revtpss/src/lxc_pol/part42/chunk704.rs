//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 704/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk704<F: Float>(t4598: F, t918: F, t2848: F, t2884: F, t4571: F, t4576: F, t4581: F, t4585: F, t916: F, t1600: F, t2897: F, t923: F, t1606: F, t698: F) -> (F, F, F, F, F, F, F) {
    let t4599 = t4598 * t918;
    let t4606 = t2884 + t2848 / 9.0 + t4571 / 9.0 - 2.0 / 9.0 * t4576 + 2.0 / 3.0 * t4581 - t4585 / 3.0;
    let t4607 = t916 * t4606;
    let t4614 = t2897 * t1600;
    let t4615 = t4614 * t918;
    let t4617 = t923 * t4606;
    let t4620 = t698 * t1606;
    (t4599, t4606, t4607, t4614, t4615, t4617, t4620)
}
