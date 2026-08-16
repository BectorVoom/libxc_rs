//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1002;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1003;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta241(t2435: f64, t5718: f64, t1893: f64, t2453: f64, t3908: f64, t1904: f64, t3895: f64, t2439: f64, t1532: f64, t2609: f64, t2626: f64, t4398: f64, t10439: f64, t162: f64, t2516: f64, t2496: f64, t2619: f64, t4302: f64, t123: f64, t1534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14290, t14293, t14294, t14296, t14297, t14312, t14328) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1002(t2435, t5718, t1893, t2453, t3908, t1904, t3895, t2439, t1532, t2609, t2626, t4398);
        let (t14330, t14334, t14336, t14339, t14362) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1003(t10439, t162, t2516, t4398, t2496, t2619, t4302, t123, t1534);
    (t14290, t14293, t14294, t14296, t14297, t14312, t14328, t14330, t14334, t14336, t14339, t14362)
}
