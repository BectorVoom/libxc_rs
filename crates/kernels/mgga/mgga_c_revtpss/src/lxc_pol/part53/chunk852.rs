//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 852/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk852<F: Float>(t1502: F, t1911: F, t2007: F, t2011: F, t28175: F, t28179: F, t28183: F, t28186: F, t28188: F, t28190: F, t28192: F, t28193: F, t28201: F, t28202: F, t28230: F, t4246: F, t569: F, t5787: F, t7221: F, t7231: F) -> (F,) {
    let t28232 = -t1502 * t7221 + t1911 * t7231 - t2007 * t4246 + t2011 * t5787 + t28230 * t569 + t28175 + t28179 - t28183 + t28186 - t28188 - t28190 + t28192 - t28193 + t28201 - t28202;
    (t28232,)
}
