//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2583/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2583(t1155: f64, t1164: f64, t21906: f64, t43689: f64, t43692: f64, t18276: f64, t4869: f64, t1238: f64, t1251: f64, t14972: f64, t1751: f64, t1761: f64, t18571: f64, t19209: f64, t19219: f64, t19234: f64, t19249: f64, t22004: f64, t22393: f64, t27784: f64, t3487: f64, t3598: f64, t4940: f64, t4945: f64, t498: f64, t5060: f64, t5089: f64, t53677: f64, t6238: f64, t6268: f64, t64595: f64, t65203: f64, t66845: f64, t66860: f64) -> (f64, f64, f64) {
    let t72104 = 0.91082604192152556044e5_f64 * t1164 * t43689 * t21906 * t43692 * t1155;
    let t72106 = 0.30762056574649219972e4_f64 * t4869 * t18276;
    let t72138 = 2.0_f64 * t1238 * t1251 * t22393 * t3598 + 3.0_f64 * t1751 * t18571 * t498 - 18.0_f64 * t19219 * t27784 * t53677 + 3.0_f64 * t4940 * t498 * t6238 - 3.0_f64 * t14972 * t6268 - 3.0_f64 * t1761 * t64595 - 6.0_f64 * t1761 * t65203 - 3.0_f64 * t1761 * t66845 - 3.0_f64 * t1761 * t66860 - 3.0_f64 * t19209 * t4945 - 6.0_f64 * t19234 * t5089 + 6.0_f64 * t19249 * t5060 + 6.0_f64 * t22004 * t3487;
    (t72104, t72106, t72138)
}
