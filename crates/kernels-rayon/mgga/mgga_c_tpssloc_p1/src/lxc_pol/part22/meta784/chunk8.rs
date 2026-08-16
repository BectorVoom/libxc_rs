//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2699/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2699(t12461: f64, t20684: f64, t20085: f64, t39655: f64, t39658: f64, t39844: f64, t5160: f64, t5356: f64, t54453: f64, t74490: f64, t74491: f64, t74493: f64, t74494: f64, t74496: f64, t74497: f64) -> (f64, f64) {
    let t75240 = t20684 * t12461;
    let t75254 = 6.0_f64 * t20085 * t5160 * t5356 - t39655 - t39658 + t39844 + t54453 - t74490 + t74491 + t74493 + t74494 + t74496 + t74497;
    (t75240, t75254)
}
