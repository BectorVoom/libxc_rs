//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1558/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1558<F: Float>(t1196: F, t12555: F, t43752: F, t43753: F, t12564: F, t3531: F, t12571: F, t3543: F, t12258: F, t698: F, t13026: F, t240: F) -> (F, F, F, F, F) {
    let t43757 = F::cast_from(0.12304822629859687989e5_f64) * t1196 * t43752 * t43753 * t12555;
    let t43759 = F::cast_from(0.23392894490538584828e1_f64) * t3531 * t12564;
    let t43761 = F::cast_from(0.10389515463408878255e3_f64) * t12571 * t3543;
    let t43762 = t698 * t12258;
    let t43764 = t240 * t13026;
    (t43757, t43759, t43761, t43762, t43764)
}
