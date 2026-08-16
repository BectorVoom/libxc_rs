//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta626 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2068;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2069;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta626<F: Float>(t99011: F, t4349: F, t93072: F, t14673: F, t92955: F, t14688: F, t4452: F, t92951: F, t14719: F, t25227: F, t2661: F, t14723: F, t25266: F, t4426: F, t1561: F, t93048: F, t14741: F, t1945: F, t807: F, t10886: F, t4416: F, t7028: F, t1549: F, t92968: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t99012, t99013, t99020, t99022, t99024, t99027, t99029) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2068::<F>(t99011, t4349, t93072, t14673, t92955, t14688, t4452, t92951, t14719, t25227, t2661, t14723);
        let (t99030, t99034, t99035, t99042, t99044, t99050) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2069::<F>(t99029, t25266, t4426, t1561, t93048, t14741, t1945, t807, t10886, t4416, t7028, t1549, t92968);
    (t99012, t99013, t99020, t99022, t99024, t99027, t99030, t99034, t99035, t99042, t99044, t99050)
}
