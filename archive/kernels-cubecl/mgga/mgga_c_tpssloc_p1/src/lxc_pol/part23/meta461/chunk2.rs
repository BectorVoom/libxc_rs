//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1350/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1350<F: Float>(t5836: F, t5842: F, t1539: F, t17800: F, t17817: F, t17863: F, t2986: F, t2994: F, t340: F, t343: F, t42861: F, t42862: F, t4531: F, t4546: F, t61365: F, t69487: F, t69503: F, t69515: F, t69540: F, t7577: F, t75836: F, t75847: F, t75912: F, t973: F, t974: F, t977: F, t978: F) -> F {
    let t76817 = t5836 * t5836;
    let t76823 = t5842 * t5842;
    let t76829 = -F::cast_from(0.16666666666666666666e-2_f64) * t973 * t977 * t2994 * t75847 - F::cast_from(0.49999999999999999999e-2_f64) * t973 * t4546 * t5836 * t5842 * t343 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t977 * t978 * t75912 + F::cast_from(0.28806584362139917695e-2_f64) * t973 * t42861 * t42862 * t75836 + F::cast_from(0.22222222222222222222e-2_f64) * t69487 - F::cast_from(0.33333333333333333332e-2_f64) * t2986 * t17800 * t7577 * t1539 + F::cast_from(0.14814814814814814814e-2_f64) * t69503 + F::cast_from(0.33333333333333333332e-2_f64) * t2986 * t17800 * t17817 - F::cast_from(0.22222222222222222222e-2_f64) * t2986 * t61365 * t17863 - F::cast_from(0.11111111111111111111e-2_f64) * t2986 * t4531 * t69515 - F::cast_from(0.11111111111111111111e-2_f64) * t69540 - F::cast_from(0.24999999999999999999e-2_f64) * t973 * t974 * t340 * t76817 * t343 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t974 * t340 * t76823 * t343;
    t76829
}
