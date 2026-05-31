//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 417/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk417<F: Float>(t71: F, t20: F, t79: F, t2863: F, t2866: F, t873: F, t80: F, t2958: F, t2960: F, t2962: F, t880: F, t861: F) -> (F, F, F, F, F, F, F) {
    let t2964 = F::cast_from(1.0_f64)/F::sqrt(t71);
    let t2966 = t2964 * t79 * t20;
    let t2967 = t2966 * t2863;
    let t2969 = t873 * t2866;
    let t2971 = t80 * t2863;
    let t2973 = -F::cast_from(0.42198333333333333333e0_f64) * t2958 + F::cast_from(0.84396666666666666666e0_f64) * t2960 + F::cast_from(0.39862222222222222223e0_f64) * t2962 + F::cast_from(0.68258333333333333333e-1_f64) * t2967 + F::cast_from(0.13651666666666666667e0_f64) * t2969 + F::cast_from(0.13692777777777777778e0_f64) * t2971;
    let t2974 = t2973 * t880;
    let t2977 = t861 * t861;
    (t2966, t2967, t2969, t2971, t2973, t2974, t2977)
}
