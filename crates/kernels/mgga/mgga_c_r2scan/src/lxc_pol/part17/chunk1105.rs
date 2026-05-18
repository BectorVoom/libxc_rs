//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1105/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1105<F: Float>(t37982: F, t7606: F, t10810: F, t2583: F, t574: F, t10757: F, t980: F, t26176: F, t37717: F, t26150: F, t37720: F, t24573: F) -> (F, F, F, F, F, F) {
    let t39785 = t37982 * t7606;
    let t39792 = t574 * t10810 * t2583;
    let t39816 = t980 * t10757;
    let t39823 = t37717 * t26176;
    let t39825 = t37720 * t26150;
    let t39827 = t37717 * t24573;
    (t39785, t39792, t39816, t39823, t39825, t39827)
}
