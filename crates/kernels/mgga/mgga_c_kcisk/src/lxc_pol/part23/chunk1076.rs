//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1076/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1076<F: Float>(t19087: F, t4406: F, t6187: F, t14995: F, t19033: F, t3952: F, t14962: F, t2326: F, t4376: F, t6204: F, t13900: F, t2321: F, t1580: F, t4381: F, t6473: F, t1588: F, t6581: F) -> (F, F, F, F, F, F) {
    let t21607 = t4406 * t19087;
    let t21608 = t6187 * t21607;
    let t21611 = t14995 * t19033;
    let t21612 = t3952 * t21611;
    let t21615 = t14962 * t2326;
    let t21616 = t21615 * t4376;
    let t21617 = t6204 * t21616;
    let t21620 = t13900 * t2321;
    let t21621 = t1580 * t21620;
    let t21626 = 0.15991812192157615301e-1 * t4381 * t6473;
    let t21631 = t1588 * t6581;
    (t21608, t21612, t21617, t21621, t21626, t21631)
}
