//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1080/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1080<F: Float>(t10845: F, t4430: F, t1558: F, t853: F, t2749: F, t2662: F, t2661: F, t4352: F, t837: F, t4416: F, t221: F, t2485: F, t4424: F) -> (F, F, F, F, F, F) {
    let t14716 = t10845 * t4430;
    let t14718 = t853 * t1558;
    let t14719 = t14718 * t2749;
    let t14720 = t2662 * t14719;
    let t14722 = F::new(0.57165357490759649296e-4) * t2661 * t14720;
    let t14723 = t4352 * t837;
    let t14724 = t2662 * t14723;
    let t14726 = F::new(0.14291339372689912324e-4) * t2661 * t14724;
    let t14727 = t4416 * t837;
    let t14728 = t2662 * t14727;
    let t14730 = F::new(0.57165357490759649296e-4) * t2661 * t14728;
    let t14732 = t2485 * t221 * t4424;
    (t14716, t14718, t14722, t14726, t14730, t14732)
}
