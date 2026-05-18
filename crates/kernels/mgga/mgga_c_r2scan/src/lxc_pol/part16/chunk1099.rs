//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1099/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1099<F: Float>(t37982: F, t7606: F, t10810: F, t2583: F, t574: F, t10757: F, t980: F, t26176: F, t37717: F, t26150: F, t37720: F, t24573: F) -> (F, F, F, F, F, F) {
    let t39785 = t37982 * t7606;
    let t39786 = F::new(0.19514881078765566037e-1) * t39785;
    let t39792 = t574 * t10810 * t2583;
    let t39793 = F::new(0.23115257973478049502e0) * t39792;
    let t39816 = t980 * t10757;
    let t39823 = t37717 * t26176;
    let t39824 = F::new(0.47609969197673950972e-2) * t39823;
    let t39825 = t37720 * t26150;
    let t39826 = F::new(0.14282990759302185292e-1) * t39825;
    let t39827 = t37717 * t24573;
    (t39786, t39793, t39816, t39824, t39826, t39827)
}
