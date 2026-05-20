//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2859/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2859<F: Float>(t1469: F, t4401: F, t61266: F, t40067: F, t40072: F, t40167: F, t40171: F, t40184: F, t50874: F, t50884: F, t77020: F, t77021: F, t77023: F, t77024: F, t77025: F, t77026: F, t77027: F, t77028: F, t77029: F) -> (F, F) {
    let t77032 = F::new(36.0) * t4401 * t61266 * t1469;
    let t77033 = t77020 + t77021 + t40067 - t40072 + t50874 + t77023 + t40167 - t40171 - t77024 - t40184 + t77025 + t77026 + t77027 + t50884 - t77028 + t77029 + t77032;
    (t77032, t77033)
}
