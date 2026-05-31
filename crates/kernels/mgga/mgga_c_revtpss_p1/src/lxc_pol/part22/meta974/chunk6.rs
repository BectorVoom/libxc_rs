//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3273/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3273<F: Float>(t10811: F, t18639: F, t10905: F, t18507: F, t10777: F, t10779: F, t2749: F, t61715: F, t18651: F, t14787: F, t18426: F, t2430: F, t2723: F, t2745: F, t2747: F, t40838: F, t4362: F, t4514: F, t50459: F, t51055: F, t51058: F, t51060: F, t6035: F, t62000: F, t62002: F) -> F {
    let t62162 = t10811 * t18639;
    let t62168 = t10905 * t18507;
    let t62176 = t10777 * t10779 * t61715 * t2749;
    let t62178 = t10811 * t18651;
    let t62186 = -F::cast_from(0.72286371995927450868e-4_f64) * t51055 + F::cast_from(0.10164000561857065645e-4_f64) * t51058 + F::cast_from(35.0_f64) / F::cast_from(18.0_f64) * t51060 - F::cast_from(0.16006300097412701803e-1_f64) * t62162 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t2747 * t50459 * t6035 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t62168 - F::cast_from(0.17149607247227894789e-1_f64) * t4514 * t62000 * t62002 * t14787 + F::cast_from(0.10164000561857065645e-3_f64) * t62176 + F::cast_from(0.20007875121765877254e-2_f64) * t62178 - F::cast_from(0.17149607247227894789e-2_f64) * t4362 * t2747 * t18426 * t2723 * t2430 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t40838;
    t62186
}
