//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta898 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2857;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2858;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta898<F: Float>(t61296: F, t61305: F, t39989: F, t40150: F, t50098: F, t50866: F, t77002: F, t77003: F, t77004: F, t77005: F, t77007: F, t77008: F, t77009: F, t77010: F, t77011: F, t77012: F, t77013: F, t162: F, t4403: F, t61037: F, t61315: F, t18259: F, t18559: F, t40172: F, t62274: F, t62276: F, t62282: F, t50888: F, t62300: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t77014, t77015, t77016) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2857::<F>(t61296, t61305, t39989, t40150, t50098, t50866, t77002, t77003, t77004, t77005, t77007, t77008, t77009, t77010, t77011, t77012, t77013);
        let (t77020, t77021, t77023, t77024, t77025, t77026, t77027, t77028, t77029) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2858::<F>(t162, t4403, t61037, t61315, t18259, t18559, t40172, t62274, t62276, t62282, t50888, t62300);
    (t77014, t77015, t77016, t77020, t77021, t77023, t77024, t77025, t77026, t77027, t77028, t77029)
}
