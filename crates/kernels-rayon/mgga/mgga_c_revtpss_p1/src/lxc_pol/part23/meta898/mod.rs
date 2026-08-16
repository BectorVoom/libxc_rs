//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta898 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2857;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2858;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta898(t61296: f64, t61305: f64, t39989: f64, t40150: f64, t50098: f64, t50866: f64, t77002: f64, t77003: f64, t77004: f64, t77005: f64, t77007: f64, t77008: f64, t77009: f64, t77010: f64, t77011: f64, t77012: f64, t77013: f64, t162: f64, t4403: f64, t61037: f64, t61315: f64, t18259: f64, t18559: f64, t40172: f64, t62274: f64, t62276: f64, t62282: f64, t50888: f64, t62300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77014, t77015, t77016) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2857(t61296, t61305, t39989, t40150, t50098, t50866, t77002, t77003, t77004, t77005, t77007, t77008, t77009, t77010, t77011, t77012, t77013);
        let (t77020, t77021, t77023, t77024, t77025, t77026, t77027, t77028, t77029) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2858(t162, t4403, t61037, t61315, t18259, t18559, t40172, t62274, t62276, t62282, t50888, t62300);
    (t77014, t77015, t77016, t77020, t77021, t77023, t77024, t77025, t77026, t77027, t77028, t77029)
}
