//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 918/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk918<F: Float>(t1398: F, t543: F, t7910: F, t7301: F, t1882: F, t7274: F, t2022: F, t5658: F, t26054: F, t5722: F, t1444: F, t1883: F) -> (F, F, F, F, F, F, F, F) {
    let t27845 = t7910 * t1398 * t543;
    let t27846 = t7301 * t27845;
    let t27852 = t7274 * t1882 * t543;
    let t27853 = t7301 * t27852;
    let t27857 = t2022 * t5658 * t543;
    let t27858 = t7301 * t27857;
    let t27861 = t26054 * t5722;
    let t27864 = t1883 * t1444;
    (t27845, t27846, t27852, t27853, t27857, t27858, t27861, t27864)
}
