//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3127/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3127<F: Float>(t12046: F, t1647: F, t16551: F, t989: F, t12153: F, t4746: F, t16237: F, t359: F, t15654: F, t3286: F, t16543: F, t3046: F) -> (F, F, F, F, F, F) {
    let t55599 = t1647 * t12046;
    let t55632 = t989 * t16551;
    let t55646 = t4746 * t12153;
    let t55649 = t359 * t16237;
    let t55685 = t15654 * t3286;
    let t55701 = t3046 * t16543;
    (t55599, t55632, t55646, t55649, t55685, t55701)
}
