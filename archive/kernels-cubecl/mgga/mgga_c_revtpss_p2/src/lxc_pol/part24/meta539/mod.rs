//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1585;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1586;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta539<F: Float>(t1892: F, t6861: F, t6843: F, t1385: F, t22964: F, t5741: F, t75251: F, t2782: F, t4086: F, t543: F, t86455: F, t14192: F, t86445: F, t9994: F, t545: F, t689: F, t869: F, t4003: F, t5744: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t86470, t86506, t86552, t86563, t86575, t86582, t86586) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1585::<F>(t1892, t6861, t6843, t1385, t22964, t5741, t75251, t2782, t4086, t543, t86455, t14192, t86445, t9994);
        let (t86597, t86604, t86608, t86634) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1586::<F>(t22964, t545, t689, t869, t2782, t4086, t543, t86506, t86445, t4003, t5744, t86470);
    (t86506, t86552, t86563, t86575, t86582, t86586, t86597, t86604, t86608, t86634)
}
