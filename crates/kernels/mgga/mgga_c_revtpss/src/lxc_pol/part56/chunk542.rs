//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 542/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk542<F: Float>(t4606: F, t916: F, t1600: F, t2897: F, t918: F, t923: F, t1606: F, t698: F, t2908: F, t4574: F, t141: F, t4579: F, t930: F) -> (F, F, F, F, F, F) {
    let t4607 = t916 * t4606;
    let t4614 = t2897 * t1600;
    let t4615 = t4614 * t918;
    let t4617 = t923 * t4606;
    let t4620 = t698 * t1606;
    let t4622 = t2908 * t4574;
    let t4623 = t141 * t4622;
    let t4625 = t930 * t4579;
    (t4607, t4615, t4617, t4620, t4623, t4625)
}
