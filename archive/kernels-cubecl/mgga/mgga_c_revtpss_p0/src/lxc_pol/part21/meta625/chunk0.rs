//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2386/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2386<F: Float>(t10292: F, t65: F, t235: F, t2710: F, t826: F, t225: F, t785: F, t2737: F, t2694: F, t9789: F, t853: F, t9794: F) -> (F, F, F, F, F, F) {
    let t40603 = F::cast_from(1.0_f64) / t65 / t10292;
    let t40604 = t235 * t40603;
    let t40607 = F::cast_from(0.11344944493805280483e-2_f64) * t2710 * t40604 * t826;
    let t40609 = t40603 * t785 * t225;
    let t40611 = F::cast_from(0.63807336860547134325e-3_f64) * t40609 * t2737;
    let t40625 = t9789 * t2694;
    let t40627 = t9794 * t853;
    (t40604, t40607, t40609, t40611, t40625, t40627)
}
