//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1272/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1272(t209: f64, t35414: f64, t35460: f64, t35505: f64, t35547: f64, t35586: f64, t35625: f64, t35666: f64, t35710: f64, t10091: f64, t1096: f64, t11721: f64, t12002: f64, t13296: f64, t15436: f64, t24004: f64, t24007: f64, t2464: f64, t2470: f64, t34303: f64, t34308: f64, t34313: f64, t35369: f64, t35375: f64, t35378: f64, t3746: f64, t7056: f64) -> (f64, f64) {
    let t35714 = (t35414 + t35460 + t35505 + t35547 + t35586 + t35625 + t35666 + t35710) * t209;
    let t35717 = 24.0_f64 * t13296 * t2470 * t3746 - 12.0_f64 * t10091 * t24007 - 2.0_f64 * t1096 * t24004 + 8.0_f64 * t11721 * t7056 - 2.0_f64 * t12002 * t2464 + 2.0_f64 * t15436 * t3746 + t34303 - t34308 - t34313 + t35369 + t35375 - t35378 - t35714;
    (t35714, t35717)
}
