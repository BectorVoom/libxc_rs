//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta486<F: Float>(t20849: F, t3754: F, t3781: F, t6564: F, t3766: F, t17191: F, t5219: F, t3566: F, t6695: F, t487: F, t69636: F, t17306: F, t1811: F) -> (F, F, F, F, F, F, F) {
        let (t72270, t72326, t72370, t72386, t72767, t72802, t72874) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1479::<F>(t20849, t3754, t3781, t6564, t3766, t17191, t5219, t3566, t6695, t487, t69636, t17306, t1811);
    (t72270, t72326, t72370, t72386, t72767, t72802, t72874)
}
