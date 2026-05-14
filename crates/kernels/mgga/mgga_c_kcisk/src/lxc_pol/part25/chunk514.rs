//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 514/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk514<F: Float>(t1664: F, t4705: F, t4704: F, t4636: F, t571: F, t4624: F, t1653: F, t4652: F, t1379: F, t311: F, t579: F, t1660: F, t827: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4706 = t4705 * t1664;
    let t4708 = 2.0 * t4704 * t4706;
    let t4711 = 0.39862222222222222223e0 * t4636;
    let t4716 = 1.0/f64::sqrt(t571);
    let t4717 = t4716 * t4624;
    let t4719 = t1653 * t4652;
    let t4722 = t311 * t1379 * t579;
    let t4723 = 0.13692777777777777778e0 * t4722;
    let t4724 = t827 * t1660;
    (t4706, t4708, t4711, t4716, t4717, t4719, t4722, t4723, t4724)
}
