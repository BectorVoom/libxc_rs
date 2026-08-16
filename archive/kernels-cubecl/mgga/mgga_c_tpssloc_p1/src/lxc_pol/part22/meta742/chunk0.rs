//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2452/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2452<F: Float>(t17863: F, t2986: F, t48279: F, t10231: F, t21409: F, t973: F, t21462: F, t2970: F, t10186: F, t1597: F, t17841: F, t21410: F, t21419: F, t21444: F, t21463: F, t2960: F, t343: F, t4518: F, t4540: F, t4546: F, t48067: F, t5836: F, t61288: F, t61291: F, t61294: F, t67060: F, t68458: F, t68554: F, t977: F, t978: F, t984: F) -> F {
    let t69699 = t2986 * t48279 * t17863;
    let t69727 = t973 * t10231 * t21409;
    let t69739 = t973 * t2970 * t21462;
    let t69741 = -F::cast_from(0.37037037037037037037e-3_f64) * t69699 + F::cast_from(0.22222222222222222222e-2_f64) * t10186 * t21419 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4518 * t68554 - F::cast_from(0.16666666666666666666e-2_f64) * t2986 * t4518 * t68458 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4546 * t21444 * t984 * t343 - F::cast_from(0.24999999999999999999e-2_f64) * t973 * t4546 * t17841 * t1597 * t343 - F::cast_from(0.24999999999999999999e-2_f64) * t973 * t4546 * t5836 * t4540 * t343 + F::cast_from(0.59259259259259259256e-2_f64) * t2960 * t21410 - F::cast_from(0.7407407407407407407e-3_f64) * t69727 + F::cast_from(0.11111111111111111111e-2_f64) * t61288 - F::cast_from(0.74074074074074074072e-3_f64) * t61291 - F::cast_from(0.55555555555555555554e-3_f64) * t61294 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t977 * t978 * t67060 - F::cast_from(0.74074074074074074072e-3_f64) * t2960 * t21463 + F::cast_from(0.9259259259259259259e-4_f64) * t69739 + t48067;
    t69741
}
