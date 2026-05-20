//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1639/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1639<F: Float>(t14224: F, t4100: F, t2782: F, t10014: F, t5741: F, t13790: F, t1398: F) -> (F, F, F, F) {
    let t14225 = t4100 * t14224;
    let t14227 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14225;
    let t14229 = F::cast_from(0.19514881078765566038e-1_f64) * t10014 * t5741;
    let t14230 = t13790 * t1398;
    (t14225, t14227, t14229, t14230)
}
