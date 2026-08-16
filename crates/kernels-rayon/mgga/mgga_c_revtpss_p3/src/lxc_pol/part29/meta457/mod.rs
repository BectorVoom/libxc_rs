//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta457 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1703;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1704;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1705;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta457(t1923: f64, t26205: f64, t2048: f64, t25102: f64, t25110: f64, t25114: f64, t25117: f64, t25120: f64, t25150: f64, t25159: f64, t25162: f64, t26170: f64, t26172: f64, t26175: f64, t26180: f64, t26182: f64, t26185: f64, t26187: f64, t26190: f64, t6954: f64, t6960: f64, t6963: f64, t7343: f64, t7352: f64, t5: f64, t117: f64, t2055: f64, t3813: f64, t670: f64, t7474: f64, t122: f64, t2097: f64, t72: f64, t25900: f64, t25904: f64, t3916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26207, t26208) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1703(t1923, t26205, t2048, t25102, t25110, t25114, t25117, t25120, t25150, t25159, t25162, t26170, t26172, t26175, t26180, t26182, t26185, t26187, t26190, t6954, t6960, t6963, t7343, t7352);
        let (t26209, t26210, t26218, t26223, t26230) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1704(t5, t26208, t117, t2055, t3813, t670, t7474, t122, t2097, t72);
        let t26231 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1705(t25900, t26230);
        let (t26232, t26234) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1706(t25904, t26231, t26230, t3916);
    (t26207, t26209, t26210, t26218, t26223, t26230, t26231, t26232, t26234)
}
