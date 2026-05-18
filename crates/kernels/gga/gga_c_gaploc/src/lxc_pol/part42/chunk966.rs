//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 966/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk966<F: Float>(t3720: F, t8469: F, t14364: F, t835: F, t723: F, t325: F, t701: F, t12161: F, t12177: F, t12182: F, t14357: F, t14384: F, t14388: F, t14391: F, t1445: F, t1457: F, t1998: F, t2004: F, t2087: F, t2103: F, t3009: F, t3040: F, t43425: F, t45229: F, t45232: F, t45234: F, t45238: F, t45242: F, t45243: F, t45247: F, t4614: F, t5771: F, t807: F, t813: F, t833: F) -> (F, F, F, F, F) {
    let t50043 = t8469 * t3720;
    let t50050 = t835 * t14364;
    let t50051 = t50050 * t723;
    let t50062 = t325 * t14364;
    let t50063 = t50062 * t701;
    let t50074 = -F::new(0.13803453343411469884e2) * t2087 * t1445 * t3009 * t12161 + F::new(0.30674340763136599741e2) * t833 * t4614 * t14384 - F::new(0.12269736305254639897e2) * t813 * t4614 * t14391 + F::new(0.14300195980740170668e1) * t5771 * t14388 + F::new(0.14300195980740170668e1) * t2103 * t1457 * t50043 - t45229 - t45232 + F::new(0.57514388930881124514e0) * t45234 - F::new(0.85206502119823888169e0) * t43425 - F::new(0.51762950037793012063e1) * t45238 + t45242 - t45243 + t45247 + F::new(0.71500979903700853338e0) * t2103 * t1457 * t50051 - F::new(0.18404604457881959845e2) * t2087 * t4614 * t14357 + F::new(0.71500979903700853338e0) * t12182 * t3040 + F::new(0.71500979903700853338e0) * t12177 * t3040 + F::new(0.35750489951850426669e0) * t2004 * t1457 * t50063 + F::new(0.23005755572352449806e1) * t807 * t1445 * t50063 - F::new(0.23005755572352449806e1) * t1998 * t1445 * t50050 * t701;
    (t50043, t50051, t50062, t50063, t50074)
}
