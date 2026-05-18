//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 898/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk898<F: Float>(t45303: F, t44712: F, t701: F, t6066: F, t7630: F, t10667: F, t10831: F, t11001: F, t13538: F, t13613: F, t13688: F, t1445: F, t2087: F, t2103: F, t3009: F, t3046: F, t43442: F, t43458: F, t43462: F, t44889: F, t45264: F, t45269: F, t45277: F, t45285: F, t45287: F, t45288: F, t45298: F, t45300: F, t4673: F, t4820: F, t5771: F, t5782: F, t7513: F, t8775: F) -> (F, F) {
    let t45304 = F::new(0.14896037479937677779e-1) * t45303;
    let t45305 = t44712 * t701;
    let t45308 = F::new(0.71500979903700853338e0) * t7630 * t6066 * t45305;
    let t45309 = -F::new(0.13803453343411469884e2) * t5782 * t13613 - t45264 + F::new(0.95334639871601137787e0) * t2103 * t4673 * t13538 - t45269 + F::new(0.71500979903700853338e0) * t3046 * t11001 + F::new(0.14300195980740170668e1) * t5771 * t13688 - t45277 - F::new(0.76685851907841499353e0) * t43442 - F::new(0.15889106645266856298e0) * t7513 * t4820 * t44889 + F::new(0.23833659967900284447e0) * t8775 * t10831 + F::new(0.36425779656224712193e1) * t45285 - t45287 - t45288 - F::new(0.13803453343411469884e2) * t2087 * t1445 * t3009 * t10667 + F::new(0.59584149919750711116e-1) * t43458 + F::new(0.59584149919750711116e-1) * t43462 - t45298 - t45300 - t45304 - t45308;
    (t45305, t45309)
}
