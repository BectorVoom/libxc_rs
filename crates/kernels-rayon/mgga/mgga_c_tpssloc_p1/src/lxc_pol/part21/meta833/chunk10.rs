//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2951/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2951(t13822: f64, t17777: f64, t973: f64, t10186: f64, t10263: f64, t13798: f64, t13839: f64, t1597: f64, t17857: f64, t17860: f64, t17864: f64, t2978: f64, t2986: f64, t2994: f64, t3008: f64, t343: f64, t4546: f64, t48336: f64, t48338: f64, t48342: f64, t55723: f64, t5829: f64, t5836: f64, t59751: f64, t61065: f64, t977: f64, t984: f64) -> f64 {
    let t61472 = t973 * t13822 * t17777;
    let t61485 = -0.14814814814814814814e-2_f64 * t61065 * t2978 * t1597 * t984 * t13839 - 0.1037037037037037037e-1_f64 * t2986 * t13798 * t59751 - 0.39506172839506172838e-2_f64 * t10186 * t17857 - 0.46090534979423868312e-2_f64 * t10186 * t17860 + 0.19753086419753086419e-2_f64 * t10186 * t17864 - 0.6172839506172839506e-3_f64 * t48336 - 0.19753086419753086419e-2_f64 * t48338 - 0.54320987654320987651e-2_f64 * t48342 - 0.55555555555555555554e-3_f64 * t61472 - 0.83333333333333333332e-3_f64 * t973 * t4546 * t5836 * t3008 * t343 - 0.11111111111111111111e-2_f64 * t973 * t977 * t2994 * t55723 + 0.27160493827160493826e-2_f64 * t10263 * t5829;
    t61485
}
