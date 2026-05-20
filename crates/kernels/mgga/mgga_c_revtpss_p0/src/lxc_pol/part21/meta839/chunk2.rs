//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3148/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3148<F: Float>(t16857: F, t3399: F, t12322: F, t5071: F, t1134: F, t16926: F, t3407: F, t56159: F, t56163: F, t56167: F, t58029: F, t58032: F, t58035: F, t58038: F, t58041: F, t58044: F) -> (F, F, F, F) {
    let t58046 = t16857 * t3399;
    let t58048 = t5071 * t12322;
    let t58051 = t3407 * t16926 * t1134;
    let t58053 = F::new(0.929655e1) * t56159 + F::new(0.103295e1) * t56163 + F::new(0.123954e2) * t56167 + F::new(0.187551e1) * t58029 + F::cast_from(0.13892666666666666667e0_f64) * t58032 - F::new(0.62517e0) * t58035 + F::cast_from(0.794188125e1_f64) * t58038 - F::cast_from(0.473371875e0_f64) * t58041 - F::new(0.52945875e1) * t58044 - F::new(0.52945875e1) * t58046 - F::new(0.17648625e1) * t58048 + F::new(0.94674375e0) * t58051;
    (t58046, t58048, t58051, t58053)
}
