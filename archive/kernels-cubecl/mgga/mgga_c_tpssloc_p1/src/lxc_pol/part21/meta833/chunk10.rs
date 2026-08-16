//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2951/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2951<F: Float>(t13822: F, t17777: F, t973: F, t10186: F, t10263: F, t13798: F, t13839: F, t1597: F, t17857: F, t17860: F, t17864: F, t2978: F, t2986: F, t2994: F, t3008: F, t343: F, t4546: F, t48336: F, t48338: F, t48342: F, t55723: F, t5829: F, t5836: F, t59751: F, t61065: F, t977: F, t984: F) -> F {
    let t61472 = t973 * t13822 * t17777;
    let t61485 = -F::cast_from(0.14814814814814814814e-2_f64) * t61065 * t2978 * t1597 * t984 * t13839 - F::cast_from(0.1037037037037037037e-1_f64) * t2986 * t13798 * t59751 - F::cast_from(0.39506172839506172838e-2_f64) * t10186 * t17857 - F::cast_from(0.46090534979423868312e-2_f64) * t10186 * t17860 + F::cast_from(0.19753086419753086419e-2_f64) * t10186 * t17864 - F::cast_from(0.6172839506172839506e-3_f64) * t48336 - F::cast_from(0.19753086419753086419e-2_f64) * t48338 - F::cast_from(0.54320987654320987651e-2_f64) * t48342 - F::cast_from(0.55555555555555555554e-3_f64) * t61472 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4546 * t5836 * t3008 * t343 - F::cast_from(0.11111111111111111111e-2_f64) * t973 * t977 * t2994 * t55723 + F::cast_from(0.27160493827160493826e-2_f64) * t10263 * t5829;
    t61485
}
