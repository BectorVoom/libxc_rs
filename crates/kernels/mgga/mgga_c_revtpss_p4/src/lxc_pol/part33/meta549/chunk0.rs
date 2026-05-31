//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1933/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1933<F: Float>(t2035: F, t29506: F, t5920: F, t94: F, t1937: F, t7732: F, t7735: F, t21663: F, t38: F, t5868: F, t76: F, t1470: F, t4173: F) -> (F, F, F, F, F, F, F) {
    let t29507 = t29506 * t2035;
    let t29508 = t94 * t5920;
    let t29510 = F::cast_from(2.0_f64) * t29508 * t1937;
    let t29512 = F::cast_from(4.0_f64) * t7732 * t7735;
    let t29513 = t21663 * t38;
    let t29532 = t76 * t5868;
    let t29538 = t4173 * t1470;
    (t29507, t29508, t29510, t29512, t29513, t29532, t29538)
}
