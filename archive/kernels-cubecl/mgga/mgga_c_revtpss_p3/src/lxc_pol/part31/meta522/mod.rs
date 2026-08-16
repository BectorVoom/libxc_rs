//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta522 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1887;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta522<F: Float>(t27888: F, t7284: F, t7289: F, t1444: F, t7296: F, t7910: F, t27883: F, t786: F, t7286: F, t1903: F, t7274: F, t25902: F, t25905: F, t25914: F, t25919: F, t25921: F, t25941: F, t25948: F, t25951: F, t27885: F, t7295: F, t7921: F) -> (F, F, F, F, F, F, F, F) {
        let (t27889, t27891, t27896, t27899, t27900, t27902, t27903, t27907) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1887::<F>(t27888, t7284, t7289, t1444, t7296, t7910, t27883, t786, t7286, t1903, t7274, t25902, t25905, t25914, t25919, t25921, t25941, t25948, t25951, t27885, t7295, t7921);
    (t27889, t27891, t27896, t27899, t27900, t27902, t27903, t27907)
}
