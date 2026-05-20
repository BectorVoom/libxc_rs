//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta722 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2484;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2485;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta722<F: Float>(t5618: F, t9784: F, t820: F, t844: F, t9991: F, t13776: F, t9775: F, t46644: F, t5622: F, t5614: F, t9779: F, t40488: F, t5610: F, t2659: F, t4086: F, t816: F, t1412: F, t808: F, t1389: F, t14224: F, t46835: F, t13769: F, t2453: F, t547: F, t9794: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48833, t48836, t48848, t48849, t48851, t48853) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2484::<F>(t5618, t9784, t820, t844, t9991, t13776, t9775, t46644, t5622, t5614, t9779, t40488, t5610);
        let (t48862, t48863, t48869, t48872) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2485::<F>(t2659, t4086, t816, t1412, t808, t1389, t14224, t46835, t13769, t2453, t547, t9794);
    (t48833, t48836, t48848, t48849, t48851, t48853, t48862, t48863, t48869, t48872)
}
