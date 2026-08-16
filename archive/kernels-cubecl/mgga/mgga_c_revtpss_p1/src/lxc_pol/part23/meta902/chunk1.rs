//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2880/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2880<F: Float>(t39807: F, t39813: F, t39818: F, t39823: F, t40084: F, t40088: F, t76977: F, t76978: F, t76980: F, t76986: F, t76987: F, t1544: F, t23111: F, t23148: F, t2403: F, t2404: F, t40131: F, t40137: F, t50080: F, t61139: F, t76999: F, t77000: F, t77002: F, t77003: F, t77004: F, t77005: F) -> (F, F) {
    let t77387 = t39807 - t39813 + t76977 - t39818 - t39823 - t76978 + t40084 + t76980 + t76986 + t40088 - t76987;
    let t77400 = F::cast_from(9.0_f64) * t1544 * t2403 * t61139 + F::cast_from(3.0_f64) * t23148 * t2403 * t2404 + F::cast_from(18.0_f64) * t23111 * t50080 - t40131 - t40137 - t76999 + t77000 + t77002 - t77003 + t77004 + t77005;
    (t77387, t77400)
}
