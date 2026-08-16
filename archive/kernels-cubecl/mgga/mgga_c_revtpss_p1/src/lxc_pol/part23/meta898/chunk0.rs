//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2857/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2857<F: Float>(t61296: F, t61305: F, t39989: F, t40150: F, t50098: F, t50866: F, t77002: F, t77003: F, t77004: F, t77005: F, t77007: F, t77008: F, t77009: F, t77010: F, t77011: F, t77012: F, t77013: F) -> (F, F, F) {
    let t77014 = F::cast_from(0.51947577317044391276e2_f64) * t61296;
    let t77015 = F::cast_from(36.0_f64) * t61305;
    let t77016 = t77002 - t77003 + t77004 + t77005 + t77007 + t77008 + t50098 + t77009 - t39989 + t40150 + t77010 - t77011 - t77012 - t77013 - t77014 + t77015 + t50866;
    (t77014, t77015, t77016)
}
