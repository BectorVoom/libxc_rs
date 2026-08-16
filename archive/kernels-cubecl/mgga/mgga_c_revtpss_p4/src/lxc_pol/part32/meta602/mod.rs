//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1937;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1938;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta602<F: Float>(t18521: F, t27261: F, t25222: F, t6030: F, t18423: F, t25234: F, t5993: F, t18414: F, t2661: F, t93082: F, t18418: F, t25227: F, t18398: F, t7045: F, t18402: F, t18409: F, t25266: F, t5980: F, t18482: F, t25270: F, t18478: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t106012, t106014, t106022, t106024, t106030, t106033) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1937::<F>(t18521, t27261, t25222, t6030, t18423, t25234, t5993, t18414, t2661, t93082, t18418, t25227);
        let (t106035, t106037, t106040, t106042, t106044, t106046) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1938::<F>(t18398, t7045, t18402, t25234, t18409, t25227, t2661, t25266, t5980, t18482, t25270, t18478, t27261);
    (t106012, t106014, t106022, t106024, t106030, t106033, t106035, t106037, t106040, t106042, t106044, t106046)
}
