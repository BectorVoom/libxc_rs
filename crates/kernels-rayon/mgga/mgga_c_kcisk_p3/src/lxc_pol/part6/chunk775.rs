//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 775/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk775(t10473: f64, t2474: f64, t1797: f64, t2507: f64, t1336: f64, t140: f64, t2522: f64, t3517: f64, t2518: f64, t1814: f64, t2372: f64, t11313: f64, t2514: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16658 = t10473 * t2474;
    let t16674 = t1797 * t2507;
    let t16676 = t140 * t1336 * t16674;
    let t16879 = t3517 * t2522;
    let t16885 = t3517 * t2518;
    let t16892 = t1814 * t2372;
    let t16897 = t11313 * t2514;
    (t16658, t16676, t16879, t16885, t16892, t16897)
}
