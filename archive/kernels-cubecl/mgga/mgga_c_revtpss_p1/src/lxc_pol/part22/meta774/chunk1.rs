//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2862/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2862<F: Float>(t13045: F, t3588: F, t371: F, t481: F, t482: F, t9291: F, t12627: F, t1284: F, t3624: F, t12640: F, t3555: F, t3781: F, t5330: F) -> (F, F, F, F, F) {
    let t44586 = t13045 * t3588;
    let t44607 = F::cast_from(0.14820648238345094262e-3_f64) * t481 * t371 * t9291 * t482;
    let t44609 = t12627 * t1284 * t3624;
    let t44624 = t12640 * t1284 * t3624;
    let t44664 = t3555 * t3781 * t5330;
    (t44586, t44607, t44609, t44624, t44664)
}
