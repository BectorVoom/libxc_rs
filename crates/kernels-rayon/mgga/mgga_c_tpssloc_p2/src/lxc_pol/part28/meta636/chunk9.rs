//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2029/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2029(t91486: f64, t12030: f64, t16452: f64, t1843: f64, t2092: f64, t24139: f64, t26224: f64, t26989: f64, t27068: f64, t3889: f64, t5215: f64, t55150: f64, t7937: f64, t81365: f64, t81375: f64, t84700: f64, t91478: f64, t91482: f64) -> f64 {
    let t93873 = 0.3289868133696452873e-1_f64 * t91486;
    let t93879 = -12.0_f64 * t26224 * t26989 * t16452 + 2.0_f64 * t27068 * t3889 + 0.3289868133696452873e-1_f64 * t81365 + 0.9869604401089358619e-1_f64 * t91478 - 0.3289868133696452873e-1_f64 * t91482 + t93873 - t55150 * t2092 - t12030 * t7937 - 0.51175726524167044691e0_f64 * t81375 - t84700 * t1843 - t5215 * t24139;
    t93879
}
