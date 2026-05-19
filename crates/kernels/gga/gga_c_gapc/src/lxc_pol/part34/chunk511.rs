//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 511/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk511<F: Float>(t2874: F, t2876: F, t2882: F, t2887: F, t2892: F, t2895: F, t2897: F, t2900: F, t2904: F, t2907: F, t2961: F, t1010: F, t575: F) -> (F, F) {
    let t2962 = -F::cast_from(0.17379648562707520765e-2_f64) * t2874 + F::cast_from(0.10427789137624512459e-2_f64) * t2876 - F::cast_from(0.10427789137624512459e-2_f64) * t2882 - F::cast_from(0.3475929712541504153e-4_f64) * t2887 + F::cast_from(0.61802030288987943842e-4_f64) * t2892 + F::cast_from(0.10427789137624512459e-2_f64) * t2895 + F::cast_from(0.13758888445476787272e-3_f64) * t2897 - F::cast_from(0.10427789137624512459e-2_f64) * t2900 - F::cast_from(0.13159621217983282916e-3_f64) * t2904 + F::cast_from(0.30368356656884499037e-4_f64) * t2907 + t2961;
    let t2964 = t1010 * t575;
    (t2962, t2964)
}
