//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta464 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1437;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta464<F: Float>(t13141: F, t1770: F, t13126: F, t1209: F, t21455: F, t5219: F, t5477: F, t5462: F, t21451: F, t17191: F, t3566: F, t13147: F) -> (F, F, F, F, F, F, F, F) {
        let (t59498, t59550, t59674, t59681, t59749, t59788, t59817, t59948) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1437::<F>(t13141, t1770, t13126, t1209, t21455, t5219, t5477, t5462, t21451, t17191, t3566, t13147);
    (t59498, t59550, t59674, t59681, t59749, t59788, t59817, t59948)
}
