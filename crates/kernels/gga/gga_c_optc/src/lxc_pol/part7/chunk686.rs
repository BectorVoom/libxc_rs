//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 686/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk686<F: Float>(t1797: F, t6742: F, t1990: F, t509: F, t1796: F, t1772: F, t603: F, t1994: F, t1909: F, t755: F, t201: F, t5: F, t743: F, t1911: F, t1916: F, t188: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6743 = t6742 * t1797;
    let t6744 = 0.32530742648344572643e-1 * t6743;
    let t6745 = t509 * t1990;
    let t6747 = 0.32530742648344572643e-1 * t1796 * t6745;
    let t6748 = t1772 * t603;
    let t6750 = 0.21687161765563048428e-1 * t1796 * t6748;
    let t6751 = t509 * t1994;
    let t6753 = 0.48159446095139119799e0 * t1796 * t6751;
    let t6754 = t1909 * t755;
    let t6756 = t5 * t6754 * t201;
    let t6757 = t743 * t6756;
    let t6760 = t1916 * t1911;
    let t6761 = t188 * t6760;
    (t6744, t6745, t6747, t6748, t6750, t6751, t6753, t6756, t6757, t6760, t6761)
}
