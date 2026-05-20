//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta781 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2588;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta781<F: Float>(t11262: F, t3711: F, t5278: F, t12640: F, t1811: F, t3766: F, t5216: F, t13141: F, t1770: F, t13126: F, t12050: F, t17710: F, t17191: F, t3555: F, t1209: F, t21455: F, t5219: F, t5477: F, t17288: F, t3754: F, t12722: F, t45785: F, t460: F, t487: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t59426, t59464, t59492, t59498, t59550, t59650) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2588::<F>(t11262, t3711, t5278, t12640, t1811, t3766, t5216, t13141, t1770, t13126, t12050, t17710);
        let (t59657, t59674, t59681, t59686, t59705, t59730) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2589::<F>(t17191, t3555, t1209, t21455, t5219, t5477, t17288, t3754, t12722, t45785, t460, t487);
    (t59426, t59464, t59492, t59498, t59550, t59650, t59657, t59674, t59681, t59686, t59705, t59730)
}
