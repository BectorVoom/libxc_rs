//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1342/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1342<F: Float>(t10352: F, t2960: F, t10232: F, t10208: F, t13822: F, t973: F, t10224: F, t2995: F, t10228: F, t10263: F, t10280: F, t23547: F, t2979: F, t2980: F, t2982: F, t2994: F, t2996: F, t3008: F, t3017: F, t39103: F, t4546: F, t977: F) -> F {
    let t42936 = t2960 * t10352;
    let t42944 = t2960 * t10232;
    let t42951 = t973 * t13822 * t10208;
    let t42962 = t973 * t10224 * t2995;
    let t42964 = t2960 * t10228;
    let t42966 = -F::cast_from(0.48888888888888888888e-1_f64) * t10263 * t3017 + F::cast_from(0.88888888888888888887e-2_f64) * t42936 + F::cast_from(0.11111111111111111111e-2_f64) * t973 * t2979 * t2980 * t39103 + F::cast_from(0.21728395061728395061e-1_f64) * t10263 * t2982 - F::cast_from(0.39506172839506172838e-2_f64) * t42944 + F::cast_from(0.23703703703703703704e-1_f64) * t2960 * t10280 - F::cast_from(0.32592592592592592591e-1_f64) * t10263 * t2996 - F::cast_from(0.33333333333333333332e-2_f64) * t42951 - F::cast_from(0.49999999999999999999e-2_f64) * t973 * t4546 * t23547 * t3008 - F::cast_from(0.16666666666666666666e-2_f64) * t973 * t977 * t2994 * t39103 + F::cast_from(0.74074074074074074072e-3_f64) * t42962 - F::cast_from(0.29629629629629629628e-2_f64) * t42964;
    t42966
}
