//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1175/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1175(t17040: f64, t17044: f64, t17054: f64, t17056: f64, t17088: f64, t17089: f64, t17096: f64, t17098: f64, t17100: f64, t1753: f64, t179: f64, t20405: f64, t20407: f64, t20409: f64, t20419: f64, t20427: f64, t20436: f64, t2592: f64, t2593: f64, t5244: f64, t5279: f64, t568: f64, t6896: f64, t6939: f64, t6961: f64) -> f64 {
    let t20438 = 0.30011812682648815881e-2_f64 * t20405 + 0.34013387707001991331e0_f64 * t20407 + 455.0_f64 / 648.0_f64 * t20409 - 0.12004725073059526352e-1_f64 * t17040 + 0.60023625365297631762e-1_f64 * t17044 + 0.13605355082800796533e0_f64 * t17054 - 0.12004725073059526352e-1_f64 * t17056 - 0.12862205435420921092e-1_f64 * t5279 * t179 * t6961 * t6939 + 0.38586616306262763276e-2_f64 * t2592 * t179 * t20419 + t17088 - 7.0_f64 / 16.0_f64 * t17089 + 0.45351183609335988443e0_f64 * t17096 - 0.68026775414003982663e-1_f64 * t17098 + 0.34013387707001991332e0_f64 * t17100 - 0.38586616306262763276e-2_f64 * t6896 * t179 * t20427 - 0.51448821741683684367e-2_f64 * t5244 * t179 * t2593 * t1753 * t568 + 0.48018900292238105409e-1_f64 * t20436;
    t20438
}
