//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 536/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk536<F: Float>(t26: F, t4727: F, t1659: F, t4644: F, t4648: F, t4638: F, t4642: F, t4646: F, t4650: F, t4672: F, t4674: F, t4711: F, t4717: F, t4719: F, t4723: F, t4724: F) -> (F, F, F, F, F, F) {
    let t4728 = t26 * t4727;
    let t4730 = t1659 * t4644;
    let t4731 = t26 * t4730;
    let t4733 = t1659 * t4648;
    let t4734 = t26 * t4733;
    let t4736 = -0.9494625e0 * t4672 + 0.1898925e1 * t4674 + t4711 + 0.19931111111111111111e0 * t4638 - 0.19931111111111111111e0 * t4642 + 0.59793333333333333334e0 * t4646 - 0.29896666666666666667e0 * t4650 + 0.15358125e0 * t4717 + 0.3071625e0 * t4719 + t4723 + 0.10954222222222222222e0 * t4724 - 0.27385555555555555556e-1 * t4728 + 0.16431333333333333333e0 * t4731 - 0.82156666666666666667e-1 * t4734;
    (t4728, t4730, t4731, t4733, t4734, t4736)
}
