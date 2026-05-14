//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 904/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk904<F: Float>(t10493: F, t10494: F, t10501: F, t10512: F, t5077: F, t5087: F, t5091: F, t5130: F, t5139: F, t5141: F, t5144: F, t5148: F, t5154: F, t7033: F, t7038: F, t7040: F) -> (F, F, F, F) {
    let t10590 = t10493 + t5077 - t10494 - t10501 + t5087 + t5091 - t5130 + t10512 - t5139 - t5141 - t5144 - t5148 - t5154;
    let t10592 = 0.51947577317044391276e2 * t7033;
    let t10593 = 0.17544670867903938621e1 * t7038;
    let t10594 = 0.35089341735807877242e1 * t7040;
    (t10590, t10592, t10593, t10594)
}
