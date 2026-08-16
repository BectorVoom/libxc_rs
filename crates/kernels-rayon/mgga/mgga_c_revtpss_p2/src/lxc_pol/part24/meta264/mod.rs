//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1035;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1036;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta264(t17376: f64, t3599: f64, t1285: f64, t17395: f64, t1781: f64, t697: f64, t1222: f64, t3367: f64, t471: f64, t372: f64, t5296: f64, t17350: f64, t3767: f64, t5277: f64, t3362: f64, t12865: f64, t5302: f64, t15904: f64, t3623: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17572, t17605, t17628, t17629, t17643, t17649, t17654) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1035(t17376, t3599, t1285, t17395, t1781, t697, t1222, t3367, t471, t372, t5296, t17350, t3767);
        let (t17661, t17687, t17693, t17694, t17708) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1036(t372, t5277, t3362, t471, t1285, t12865, t5302, t15904, t3623);
    (t17572, t17605, t17628, t17629, t17643, t17649, t17654, t17661, t17687, t17693, t17694, t17708)
}
