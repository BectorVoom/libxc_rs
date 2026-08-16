//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2619/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2619(t11697: f64, t22153: f64, t3577: f64, t13969: f64, t22274: f64, t3515: f64, t1227: f64, t22196: f64, t1222: f64, t22015: f64, t15740: f64, t18584: f64, t18965: f64, t18997: f64, t19077: f64, t3447: f64, t3578: f64, t4733: f64, t4889: f64, t52903: f64, t52995: f64, t53087: f64, t6219: f64, t66545: f64, t66554: f64, t66566: f64, t68513: f64) -> f64 {
    let t73084 = t3577 * t11697 * t22153;
    let t73096 = t3515 * t13969 * t22274;
    let t73099 = t1227 * t13969 * t22196;
    let t73102 = t22015 * t1222;
    let t73108 = -t3577 * t3578 * t6219 * t4733 / 1536.0_f64 - t73084 / 2304.0_f64 - t15740 * t18584 / 768.0_f64 - t52903 * t18965 / 288.0_f64 + t4889 * t18997 / 36.0_f64 - t53087 * t19077 / 192.0_f64 - t66545 / 81.0_f64 - t73096 / 1536.0_f64 + 5.0_f64 / 6912.0_f64 * t73099 + t66554 / 1536.0_f64 - t73102 / 288.0_f64 + t3447 * t52995 * t68513 / 12.0_f64 - t66566 / 2304.0_f64;
    t73108
}
