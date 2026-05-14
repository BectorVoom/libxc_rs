//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1053/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1053<F: Float>(t24146: F, t5289: F, t1931: F, t9020: F, t17862: F, t2564: F, t24126: F, t24128: F, t24130: F, t24132: F, t24135: F, t24138: F, t24141: F, t24144: F, t23299: F, t5322: F) -> (F, F, F, F, F) {
    let t24147 = t5289 * t24146;
    let t24149 = t1931 * t9020;
    let t24151 = t17862 * t2564;
    let t24153 = t24126 / 54.0 - t24128 / 24.0 + t24130 / 3.0 + 2.0 / 9.0 * t24132 + t24135 / 3.0 - t24138 / 64.0 + t24141 / 4.0 - 11.0 / 18.0 * t24144 - t24147 / 12.0 - t24149 / 72.0 - t24151 / 8.0;
    let t24155 = t5322 * t23299;
    (t24147, t24149, t24151, t24153, t24155)
}
