//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1223/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1223<F: Float>(t1220: F, t20454: F, t6814: F, t969: F, t1835: F, t4758: F, t6820: F, t10877: F, t10893: F, t10936: F, t1212: F, t15304: F, t15362: F, t15450: F, t1831: F, t18965: F, t19042: F, t3545: F, t3585: F, t3592: F, t5211: F, t5234: F, t5247: F, t5254: F, t6789: F, t6805: F, t6808: F) -> F {
    let t20455 = t20454 * t1220;
    let t20465 = t6814 * t969;
    let t20468 = t1835 * t4758;
    let t20471 = t6820 * t969;
    let t20474 = F::new(2.0) * t15362 * t1831 + F::new(2.0) * t5211 * t5234 - F::new(2.0) * t10936 * t6789 + F::new(1.0) * t3545 * t6805 + F::new(1.0) * t1212 * t20455 + F::new(0.32164683177870697974e2) * t10893 * t6808 + t19042 - F::new(0.19751789702565206229e-1) * t18965 - F::new(0.23392893589820816284e1) * t15304 * t5247 + F::new(0.346315117987517266e2) * t15450 * t5254 + F::new(0.35089340384731224426e1) * t3592 * t20465 - F::new(0.23392893589820816284e1) * t3585 * t20468 - F::new(0.1038945353962551798e3) * t10877 * t20471;
    t20474
}
