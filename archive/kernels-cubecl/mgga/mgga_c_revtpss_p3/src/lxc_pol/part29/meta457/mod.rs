//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1703;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1704;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1705;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta457<F: Float>(t1923: F, t26205: F, t2048: F, t25102: F, t25110: F, t25114: F, t25117: F, t25120: F, t25150: F, t25159: F, t25162: F, t26170: F, t26172: F, t26175: F, t26180: F, t26182: F, t26185: F, t26187: F, t26190: F, t6954: F, t6960: F, t6963: F, t7343: F, t7352: F, t5: F, t117: F, t2055: F, t3813: F, t670: F, t7474: F, t122: F, t2097: F, t72: F, t25900: F, t25904: F, t3916: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t26207, t26208) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1703::<F>(t1923, t26205, t2048, t25102, t25110, t25114, t25117, t25120, t25150, t25159, t25162, t26170, t26172, t26175, t26180, t26182, t26185, t26187, t26190, t6954, t6960, t6963, t7343, t7352);
        let (t26209, t26210, t26218, t26223, t26230) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1704::<F>(t5, t26208, t117, t2055, t3813, t670, t7474, t122, t2097, t72);
        let t26231 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1705::<F>(t25900, t26230);
        let (t26232, t26234) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1706::<F>(t25904, t26231, t26230, t3916);
    (t26207, t26209, t26210, t26218, t26223, t26230, t26231, t26232, t26234)
}
