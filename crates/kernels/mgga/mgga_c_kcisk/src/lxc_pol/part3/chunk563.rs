//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 563/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk563<F: Float>(t1665: F, t4699: F, t1643: F, t583: F, t573: F, t1663: F, t1664: F, t4636: F, t571: F, t4624: F, t1653: F, t4652: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4701 = F::new(2.0) * t4699 * t1665;
    let t4702 = t1643 * t583;
    let t4703 = F::new(1.0) / t4702;
    let t4704 = t573 * t4703;
    let t4705 = t1663 * t1663;
    let t4706 = t4705 * t1664;
    let t4708 = F::new(2.0) * t4704 * t4706;
    let t4711 = F::new(0.39862222222222222223e0) * t4636;
    let t4716 = F::new(1.0)/f64::sqrt(t571);
    let t4717 = t4716 * t4624;
    let t4719 = t1653 * t4652;
    (t4701, t4703, t4704, t4705, t4706, t4708, t4711, t4716, t4717, t4719)
}
