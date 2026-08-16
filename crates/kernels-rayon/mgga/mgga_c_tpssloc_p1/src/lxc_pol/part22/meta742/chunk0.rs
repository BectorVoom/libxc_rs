//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2452/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2452(t17863: f64, t2986: f64, t48279: f64, t10231: f64, t21409: f64, t973: f64, t21462: f64, t2970: f64, t10186: f64, t1597: f64, t17841: f64, t21410: f64, t21419: f64, t21444: f64, t21463: f64, t2960: f64, t343: f64, t4518: f64, t4540: f64, t4546: f64, t48067: f64, t5836: f64, t61288: f64, t61291: f64, t61294: f64, t67060: f64, t68458: f64, t68554: f64, t977: f64, t978: f64, t984: f64) -> f64 {
    let t69699 = t2986 * t48279 * t17863;
    let t69727 = t973 * t10231 * t21409;
    let t69739 = t973 * t2970 * t21462;
    let t69741 = -0.37037037037037037037e-3_f64 * t69699 + 0.22222222222222222222e-2_f64 * t10186 * t21419 - 0.16666666666666666666e-2_f64 * t2986 * t4518 * t68554 - 0.16666666666666666666e-2_f64 * t2986 * t4518 * t68458 - 0.83333333333333333332e-3_f64 * t973 * t4546 * t21444 * t984 * t343 - 0.24999999999999999999e-2_f64 * t973 * t4546 * t17841 * t1597 * t343 - 0.24999999999999999999e-2_f64 * t973 * t4546 * t5836 * t4540 * t343 + 0.59259259259259259256e-2_f64 * t2960 * t21410 - 0.7407407407407407407e-3_f64 * t69727 + 0.11111111111111111111e-2_f64 * t61288 - 0.74074074074074074072e-3_f64 * t61291 - 0.55555555555555555554e-3_f64 * t61294 + 0.27777777777777777777e-3_f64 * t973 * t977 * t978 * t67060 - 0.74074074074074074072e-3_f64 * t2960 * t21463 + 0.9259259259259259259e-4_f64 * t69739 + t48067;
    t69741
}
