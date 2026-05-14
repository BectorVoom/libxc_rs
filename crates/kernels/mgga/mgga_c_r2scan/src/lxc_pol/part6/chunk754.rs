//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 754/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk754<F: Float>(t4842: F, t4845: F, t4873: F, t5016: F, t5020: F, t5022: F, t5024: F, t5026: F, t5028: F, t5030: F, t5033: F, t5035: F, t5039: F, t108: F, t4937: F, t4985: F, t5014: F) -> (F,) {
    let t5040 = t5016 - t5020 + t4842 + t5022 - t4845 + t5024 + t5026 - t5028 - t5030 + t4873 + t5033 + t5035 + t5039;
    let t5043 = (t4937 + t4985 + t5014 + t5040) * t108;
    (t5043,)
}
