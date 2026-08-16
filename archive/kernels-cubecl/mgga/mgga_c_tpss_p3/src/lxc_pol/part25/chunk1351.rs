//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1351/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1351<F: Float>(t65624: F, t65634: F, t65647: F, t65650: F, t67175: F, t67183: F, t67185: F, t69551: F, t69553: F, t69555: F, t69558: F, t69561: F, t69564: F) -> F {
    let t71807 = -t69551 / F::cast_from(768.0_f64) - F::cast_from(35.0_f64) / F::cast_from(288.0_f64) * t69553 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t69555 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t65624 + t67175 - t65634 - t69558 / F::cast_from(192.0_f64) - t69561 / F::cast_from(2.0_f64) + t69564 / F::cast_from(4.0_f64) - t67183 + t67185 - F::cast_from(119.0_f64) / F::cast_from(432.0_f64) * t65647 - t65650;
    t71807
}
