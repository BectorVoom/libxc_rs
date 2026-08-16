//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 922/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk922<F: Float>(t7024: F, t83: F, t5077: F, t5091: F, t5130: F, t5139: F, t5141: F, t5148: F, t7013: F, t7015: F, t7017: F, t7018: F, t7019: F, t7020: F, t7021: F, t7022: F, t7023: F) -> (F, F) {
    let t7025 = t83 * t7024;
    let t7026 = t5077 - t7013 + t7015 - t7017 - t7018 - t7019 + t5091 - t5130 - t7020 - t7021 - t5139 - t5141 + t7022 - t5148 + t7023 + t7025;
    (t7025, t7026)
}
