//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2783/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2783<F: Float>(t2682: F, t2719: F, t4368: F, t820: F, t10778: F, t221: F, t10777: F, t14792: F, t2659: F, t4503: F, t816: F, t14803: F, t50769: F) -> (F, F, F, F) {
    let t51121 = t820 * t2719 * t2682 * t4368;
    let t51122 = F::cast_from(0.34013387707001991332e-1_f64) * t51121;
    let t51123 = t10778 * t221;
    let t51125 = t10777 * t51123 * t14792;
    let t51133 = t816 * t2659 * t4503;
    let t51135 = t51133 * t50769 * t14803;
    (t51122, t51123, t51125, t51135)
}
