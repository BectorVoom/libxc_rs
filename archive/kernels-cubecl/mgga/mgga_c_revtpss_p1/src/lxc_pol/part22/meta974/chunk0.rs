//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3267/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3267<F: Float>(t18414: F, t40799: F, t9794: F, t10760: F, t18418: F, t18392: F, t236: F, t807: F, t854: F, t18643: F, t40731: F, t10779: F, t10786: F, t14931: F, t61956: F) -> (F, F, F, F, F) {
    let t62012 = t40799 * t9794 * t18414;
    let t62015 = t10760 * t9794 * t18418;
    let t62021 = t807 * t236 * t854 * t18392;
    let t62029 = t40731 * t18643;
    let t62033 = t14931 * t10779 * t61956 * t10786;
    (t62012, t62015, t62021, t62029, t62033)
}
