//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1573;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1574;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta534<F: Float>(t22837: F, t9962: F, t22860: F, t47194: F, t22849: F, t3957: F, t13790: F, t22020: F, t2661: F, t9934: F, t177: F, t22789: F, t762: F, t72: F, t757: F, t1317: F, t22790: F, t1320: F, t512: F, t749: F, t221: F, t22954: F, t4018: F, t4019: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t85839, t85865, t85873, t85885, t85895) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1573::<F>(t22837, t9962, t22860, t47194, t22849, t3957, t13790, t22020, t2661, t9934, t177, t22789, t762);
        let (t85912, t85929, t85931, t85986, t86061) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1574::<F>(t22789, t72, t757, t1317, t22790, t1320, t512, t749, t221, t22954, t4018, t4019);
    (t85839, t85865, t85873, t85885, t85895, t85912, t85929, t85931, t85986, t86061)
}
