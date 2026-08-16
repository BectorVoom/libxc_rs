//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1350/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1350(t5836: f64, t5842: f64, t1539: f64, t17800: f64, t17817: f64, t17863: f64, t2986: f64, t2994: f64, t340: f64, t343: f64, t42861: f64, t42862: f64, t4531: f64, t4546: f64, t61365: f64, t69487: f64, t69503: f64, t69515: f64, t69540: f64, t7577: f64, t75836: f64, t75847: f64, t75912: f64, t973: f64, t974: f64, t977: f64, t978: f64) -> f64 {
    let t76817 = t5836 * t5836;
    let t76823 = t5842 * t5842;
    let t76829 = -0.16666666666666666666e-2_f64 * t973 * t977 * t2994 * t75847 - 0.49999999999999999999e-2_f64 * t973 * t4546 * t5836 * t5842 * t343 + 0.27777777777777777777e-3_f64 * t973 * t977 * t978 * t75912 + 0.28806584362139917695e-2_f64 * t973 * t42861 * t42862 * t75836 + 0.22222222222222222222e-2_f64 * t69487 - 0.33333333333333333332e-2_f64 * t2986 * t17800 * t7577 * t1539 + 0.14814814814814814814e-2_f64 * t69503 + 0.33333333333333333332e-2_f64 * t2986 * t17800 * t17817 - 0.22222222222222222222e-2_f64 * t2986 * t61365 * t17863 - 0.11111111111111111111e-2_f64 * t2986 * t4531 * t69515 - 0.11111111111111111111e-2_f64 * t69540 - 0.24999999999999999999e-2_f64 * t973 * t974 * t340 * t76817 * t343 - 0.83333333333333333332e-3_f64 * t973 * t974 * t340 * t76823 * t343;
    t76829
}
