//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1547;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta520(t1160: f64, t24453: f64, t24362: f64, t3479: f64, t24407: f64, t3523: f64, t1179: f64, t24252: f64, t24864: f64, t460: f64, t5219: f64, t6695: f64, t1811: f64, t20849: f64, t6564: f64, t1770: f64, t12772: f64, t24568: f64, t5340: f64, t24572: f64, t5331: f64, t11249: f64, t24543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81791, t81836, t81873, t82050, t82147, t82150) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1547(t1160, t24453, t24362, t3479, t24407, t3523, t1179, t24252, t24864, t460, t5219, t6695);
        let (t82204, t82217, t82238, t82286, t82289, t82293) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1548(t1811, t20849, t6564, t1770, t6695, t12772, t24568, t5340, t24572, t5331, t11249, t24543);
    (t81791, t81836, t81873, t82050, t82147, t82150, t82204, t82217, t82238, t82286, t82289, t82293)
}
