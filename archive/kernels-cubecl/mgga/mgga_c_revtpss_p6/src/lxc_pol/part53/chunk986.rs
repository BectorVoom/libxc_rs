//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 986/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk986<F: Float>(t27: F, t8464: F, t221: F, t2485: F, t257: F, t786: F, t7063: F, t1032: F, t1955: F) -> (F, F, F, F) {
    let t31743 = t8464 * t27;
    let t31746 = t2485 * t221 * t257;
    let t31747 = t786 * t31743 * t31746;
    let t31748 = F::cast_from(0.18822977838986977999e-4_f64) * t31747;
    let t31750 = t7063 * t31743 * t31746;
    let t31751 = F::cast_from(0.33467254597718846885e-4_f64) * t31750;
    let t31752 = t1955 * t1032;
    (t31746, t31748, t31751, t31752)
}
