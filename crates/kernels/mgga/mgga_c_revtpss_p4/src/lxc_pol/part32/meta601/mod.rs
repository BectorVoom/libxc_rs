//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1936;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta601<F: Float>(t18471: F, t25270: F, t18446: F, t18629: F, t18428: F, t27261: F, t18651: F, t18639: F, t18643: F, t92955: F, t18456: F, t6037: F, t92951: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t105993, t105995, t105997, t105999, t106001, t106003, t106006, t106008, t106010) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1936::<F>(t18471, t25270, t18446, t18629, t18428, t27261, t18651, t18639, t18643, t92955, t18456, t6037, t92951);
    (t105993, t105995, t105997, t105999, t106001, t106003, t106006, t106008, t106010)
}
