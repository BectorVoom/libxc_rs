//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1023;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta274<F: Float>(t1419: F, t4086: F, t786: F, t555: F, t5744: F, t2435: F, t4093: F, t4083: F, t9303: F, t2777: F, t4092: F, t2439: F, t3999: F, t123: F, t212: F, t2434: F, t4089: F, t138: F, t2438: F, t785: F, t1432: F, t2470: F, t4107: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t10014, t10022, t10023, t10032, t10035, t10044) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1023::<F>(t1419, t4086, t786, t555, t5744, t2435, t4093, t4083, t9303, t2777, t4092, t2439);
        let (t10049, t10069, t10070, t10073, t10074, t10098) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1024::<F>(t1419, t3999, t123, t212, t2434, t4089, t138, t2438, t785, t1432, t2470, t4107);
    (t10014, t10022, t10023, t10032, t10035, t10044, t10049, t10069, t10070, t10073, t10074, t10098)
}
