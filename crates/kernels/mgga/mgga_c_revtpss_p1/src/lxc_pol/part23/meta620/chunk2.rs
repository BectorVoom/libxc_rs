//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2301/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2301<F: Float>(t247: F, t24713: F, t3719: F, t12900: F, t17629: F, t21170: F, t21189: F, t21193: F, t21216: F, t21234: F, t21249: F, t24681: F, t24684: F, t24700: F, t24706: F, t3718: F, t484: F, t5381: F, t5384: F, t6683: F) -> (F, F) {
    let t24715 = t247 * t3719 * t24713;
    let t24722 = -F::cast_from(0.53100265402527852012e-1_f64) * t24681 * t484 + F::cast_from(0.21722835846488666732e-1_f64) * t24684 * t484 + F::cast_from(0.21437009059034868486e-3_f64) * t24700 * t484 + t21170 / F::cast_from(216.0_f64) - F::cast_from(0.64311027177104605458e-3_f64) * t3718 * t24706 + t12900 + F::cast_from(0.85748036236139473944e-3_f64) * t21189 - F::cast_from(0.85748036236139473944e-3_f64) * t5381 * t6683 - F::cast_from(0.57165357490759649295e-3_f64) * t21193 + F::cast_from(0.12862205435420921092e-2_f64) * t5384 * t24715 - F::cast_from(0.57165357490759649295e-3_f64) * t21216 + t17629 / F::cast_from(432.0_f64) + F::cast_from(0.47637797908966374413e-3_f64) * t21234 + t21249 / F::cast_from(54.0_f64);
    (t24715, t24722)
}
