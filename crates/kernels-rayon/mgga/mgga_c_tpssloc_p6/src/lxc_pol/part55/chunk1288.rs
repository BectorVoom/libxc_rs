//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1288/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1288(t34353: f64, t3640: f64, t118229: f64, t118233: f64, t118251: f64, t125182: f64, t125237: f64, t125280: f64, t1254: f64, t125580: f64, t1256: f64, t125624: f64, t125668: f64, t125712: f64, t125752: f64, t1763: f64, t193: f64, t24905: f64, t24909: f64, t27834: f64, t27843: f64, t32555: f64, t32561: f64, t336: f64, t4700: f64, t5091: f64, t7394: f64, t7398: f64, t8090: f64) -> f64 {
    let t125759 = t34353 * t3640;
    let t125789 = t193 * t336 * (t125182 + t125237 + t125280 + t125580 + t125624 + t125668 + t125712 + t125752) * t1256 - t4700 * t125759 * t1254 - t4700 * t118229 * t1763 + 2.0_f64 * t4700 * t118233 * t27843 - t4700 * t32555 * t5091 - 2.0_f64 * t4700 * t24905 * t8090 + 4.0_f64 * t4700 * t24909 * t8090 * t1254 - 2.0_f64 * t4700 * t7398 * t27834 + 4.0_f64 * t4700 * t24909 * t1763 * t7394 - 6.0_f64 * t4700 * t118251 * t27843 + 2.0_f64 * t4700 * t32561 * t5091;
    t125789
}
