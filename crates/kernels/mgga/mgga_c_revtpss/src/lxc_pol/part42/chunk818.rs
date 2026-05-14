//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 818/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk818<F: Float>(t1868: F, t4140: F, t3854: F, t3859: F, t3862: F, t3867: F, t3871: F, t3873: F, t4030: F, t4035: F, t4037: F, t4042: F, t4139: F, t5634: F, t5637: F, t5639: F, t5640: F, t5641: F) -> (F,) {
    let t5783 = t4140 * t1868;
    let t5786 = 3.0 * t4139 * t5783 + t3854 + t3859 - t3862 - t3867 + t3871 + t3873 + t4030 - t4035 - t4037 + t4042 + t5634 - t5637 - t5639 - t5640 - t5641;
    (t5786,)
}
