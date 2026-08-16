//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta324 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1236;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta324(t12966: f64, t480: f64, t12621: f64, t482: f64, t371: f64, t372: f64, t12657: f64, t225: f64, t3667: f64, t3678: f64, t1236: f64, t676: f64, t1235: f64, t12627: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12967, t12970, t12972, t12975) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1236(t12966, t480, t12621, t482, t371, t372, t12657, t225);
        let (t12976, t12979, t12984, t12985, t12987) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1237(t12975, t480, t3667, t3678, t1236, t371, t676, t1235, t12627, t225);
    (t12967, t12970, t12972, t12975, t12976, t12979, t12984, t12985, t12987)
}
