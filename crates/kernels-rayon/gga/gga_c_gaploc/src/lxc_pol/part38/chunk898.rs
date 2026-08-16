//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 898/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk898(t45303: f64, t44712: f64, t701: f64, t6066: f64, t7630: f64, t10667: f64, t10831: f64, t11001: f64, t13538: f64, t13613: f64, t13688: f64, t1445: f64, t2087: f64, t2103: f64, t3009: f64, t3046: f64, t43442: f64, t43458: f64, t43462: f64, t44889: f64, t45264: f64, t45269: f64, t45277: f64, t45285: f64, t45287: f64, t45288: f64, t45298: f64, t45300: f64, t4673: f64, t4820: f64, t5771: f64, t5782: f64, t7513: f64, t8775: f64) -> (f64, f64) {
    let t45304 = 0.14896037479937677779e-1_f64 * t45303;
    let t45305 = t44712 * t701;
    let t45308 = 0.71500979903700853338e0_f64 * t7630 * t6066 * t45305;
    let t45309 = -0.13803453343411469884e2_f64 * t5782 * t13613 - t45264 + 0.95334639871601137787e0_f64 * t2103 * t4673 * t13538 - t45269 + 0.71500979903700853338e0_f64 * t3046 * t11001 + 0.14300195980740170668e1_f64 * t5771 * t13688 - t45277 - 0.76685851907841499353e0_f64 * t43442 - 0.15889106645266856298e0_f64 * t7513 * t4820 * t44889 + 0.23833659967900284447e0_f64 * t8775 * t10831 + 0.36425779656224712193e1_f64 * t45285 - t45287 - t45288 - 0.13803453343411469884e2_f64 * t2087 * t1445 * t3009 * t10667 + 0.59584149919750711116e-1_f64 * t43458 + 0.59584149919750711116e-1_f64 * t43462 - t45298 - t45300 - t45304 - t45308;
    (t45305, t45309)
}
