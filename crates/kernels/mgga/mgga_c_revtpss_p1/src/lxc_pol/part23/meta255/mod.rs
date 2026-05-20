//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1441;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1442;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta255<F: Float>(t1444: F, t2434: F, t123: F, t3915: F, t1359: F, t9292: F, t1363: F, t9288: F, t1362: F, t3911: F, t3920: F, t2237: F, t240: F, t550: F, t816: F, t1379: F, t2689: F, t3952: F, t547: F, t9646: F, t2236: F, t66: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9685, t9686, t9687, t9691, t9692, t9694, t9695, t9707) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1441::<F>(t1444, t2434, t123, t3915, t1359, t9292, t1363, t9288, t1362, t3911, t3920, t2237, t240);
        let (t9711, t9712, t9718, t9720) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1442::<F>(t550, t816, t9707, t1379, t2689, t3952, t547, t9646, t2236, t66);
    (t9685, t9686, t9687, t9691, t9692, t9694, t9695, t9707, t9711, t9712, t9718, t9720)
}
