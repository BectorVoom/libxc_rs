//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta158 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk793;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk794;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta158(t225: f64, t385: f64, t6343: f64, t1695: f64, t3269: f64, t1082: f64, t6244: f64, t1089: f64, t6271: f64, t1651: f64, t5004: f64, t6258: f64, t378: f64, t6305: f64, t3304: f64, t1668: f64, t1678: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6345, t6350) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk793(t225, t385, t6343, t1695);
        let t6351 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk794(t3269, t6350);
        let (t6362, t6365, t6368, t6371, t6374, t6375, t6379) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk795(t1082, t6244, t1089, t6271, t1651, t5004, t6258, t378, t6305, t3304, t1668, t1678);
    (t6345, t6350, t6351, t6362, t6365, t6368, t6371, t6374, t6375, t6379)
}
