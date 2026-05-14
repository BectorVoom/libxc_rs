//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 873/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk873<F: Float>(t2970: F, t6022: F, t2111: F, t751: F, t2036: F, t785: F, t2155: F, t314: F, t204: F, t334: F, t3981: F, t1281: F, t824: F) -> (F, F, F, F, F, F, F) {
    let t6023 = t2970 * t6022;
    let t6031 = t751 * t2111;
    let t6036 = t2036 * t785;
    let t6065 = 1.0 / t2155 / t314;
    let t6087 = t204 * t3981 * t334;
    let t6088 = 0.55403703703703703703e-1 * t6087;
    let t6090 = t204 * t1281 * t824;
    (t6023, t6031, t6036, t6065, t6087, t6088, t6090)
}
