//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2905/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2905<F: Float>(t41361: F, t41363: F, t41369: F, t51978: F, t51981: F, t51984: F, t51987: F, t51990: F, t51995: F, t52000: F, t52004: F, t52013: F, t52016: F, t52020: F, t52023: F, t52025: F, t52028: F, t52031: F, t52033: F, t52035: F, t52037: F, t52039: F, t52041: F) -> (F, F) {
    let t52716 = F::cast_from(0.31003950617283950619e0_f64) * t51978 - F::cast_from(0.85199506172839506175e-1_f64) * t51981 + F::cast_from(0.49293999999999999999e0_f64) * t51984 + F::cast_from(0.16431333333333333333e0_f64) * t51987 + F::cast_from(0.49293999999999999999e0_f64) * t51990 + F::new(0.49294e0) * t51995 + F::cast_from(0.43816888888888888889e0_f64) * t52000 - F::new(0.147882e1) * t52004 + F::cast_from(0.93011851851851851855e0_f64) * t41361 + F::cast_from(0.79724444444444444447e0_f64) * t41363 - F::cast_from(0.39862222222222222222e0_f64) * t41369;
    let t52729 = -F::cast_from(0.10954222222222222222e0_f64) * t52013 + F::cast_from(0.49293999999999999999e0_f64) * t52016 - F::new(0.147882e1) * t52020 + F::cast_from(0.427258125e1_f64) * t52023 - F::cast_from(0.230371875e0_f64) * t52025 + F::cast_from(0.35876000000000000001e1_f64) * t52028 + F::cast_from(0.39862222222222222223e1_f64) * t52031 + F::new(0.17938e1) * t52033 + F::cast_from(0.79724444444444444445e0_f64) * t52035 - F::cast_from(0.26574814814814814816e0_f64) * t52037 - F::cast_from(0.11958666666666666667e1_f64) * t52039 - F::cast_from(0.59793333333333333333e0_f64) * t52041;
    (t52716, t52729)
}
