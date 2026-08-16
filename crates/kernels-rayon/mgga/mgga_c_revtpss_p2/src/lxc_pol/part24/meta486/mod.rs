//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta486(t20849: f64, t3754: f64, t3781: f64, t6564: f64, t3766: f64, t17191: f64, t5219: f64, t3566: f64, t6695: f64, t487: f64, t69636: f64, t17306: f64, t1811: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t72270, t72326, t72370, t72386, t72767, t72802, t72874) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1479(t20849, t3754, t3781, t6564, t3766, t17191, t5219, t3566, t6695, t487, t69636, t17306, t1811);
    (t72270, t72326, t72370, t72386, t72767, t72802, t72874)
}
