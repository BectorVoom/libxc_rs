//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk928;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk929;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta184(t1389: f64, t3964: f64, t9732: f64, t2735: f64, t546: f64, t1353: f64, t1412: f64, t808: f64, t1369: f64, t2699: f64, t1372: f64, t3943: f64, t794: f64, t3946: f64, t159: f64, t216: f64, t124: f64, t800: f64, t9400: f64, t3989: f64, t4014: f64, t1370: f64, t9697: f64, t9700: f64, t9705: f64, t9711: f64, t9712: f64, t9716: f64, t9725: f64, t9729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9735, t9736, t9737, t9738, t9739, t9741, t9742, t9744) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk928(t1389, t3964, t9732, t2735, t546, t1353, t1412, t808, t1369, t2699, t1372, t3943, t794);
        let (t9747, t9748, t9750, t9755) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk929(t3946, t9744, t1412, t159, t216, t124, t800, t9400, t3989, t4014, t1370, t9697, t9700, t9705, t9711, t9712, t9716, t9725, t9729, t9735, t9739, t9742);
    (t9736, t9737, t9738, t9741, t9744, t9747, t9748, t9750, t9755)
}
