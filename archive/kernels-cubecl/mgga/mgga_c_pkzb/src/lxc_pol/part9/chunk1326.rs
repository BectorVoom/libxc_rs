//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1326/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1326<F: Float>(t1245: F, t6455: F, t8480: F, t914: F, t6523: F, t10121: F, t1250: F, t19090: F, t19106: F, t19115: F, t19264: F, t22007: F, t22966: F, t22980: F, t23076: F, t23082: F, t23394: F, t23412: F, t23446: F, t23450: F, t2363: F, t2387: F, t2393: F, t2439: F, t2970: F, t3258: F, t3260: F, t3266: F, t3269: F, t3270: F, t397: F, t6566: F, t6571: F, t6574: F, t6592: F, t7832: F, t8507: F, t8511: F, t8515: F, t8516: F, t8533: F, t8536: F, t8539: F, t8543: F, t946: F) -> F {
    let t23485 = t6455 * t1245;
    let t23498 = t914 * t8480;
    let t23504 = t6523 * t1245;
    let t23535 = F::cast_from(0.19756347548806534796e1_f64) * t2439 * t8533 + F::cast_from(0.11853808529283920877e2_f64) * t8507 * t7832 * t10121 * t2387 + F::cast_from(0.19756347548806534796e1_f64) * t6455 * t8511 * t8543 + F::cast_from(0.65854491829355115987e0_f64) * t23485 * t6592 + F::cast_from(0.39512695097613069591e1_f64) * t2363 * t23450 * t3260 + F::cast_from(0.39512695097613069591e1_f64) * t8516 * t6566 - F::cast_from(0.65854491829355115987e0_f64) * t3269 * t2970 * t22966 + F::cast_from(0.65854491829355115987e0_f64) * t19264 * t1250 + F::cast_from(0.19756347548806534796e1_f64) * t23498 * t946 - F::cast_from(0.39512695097613069591e1_f64) * t2393 * t23412 * t3270 - F::cast_from(0.39512695097613069591e1_f64) * t23504 * t6571 + F::cast_from(0.15805078039045227836e2_f64) * t19106 * t3258 * t22007 * t23076 - F::cast_from(0.23707617058567841754e2_f64) * t19115 * t3258 * t22007 * t23082 + F::cast_from(0.19756347548806534796e1_f64) * t6455 * t8515 * t8543 + F::cast_from(0.65854491829355115987e0_f64) * t397 * t23394 - F::cast_from(0.19756347548806534796e1_f64) * t8536 * t8539 - F::cast_from(0.19756347548806534796e1_f64) * t2393 * t1245 * t2387 * t3270 - F::cast_from(0.19756347548806534796e1_f64) * t2393 * t23446 * t3270 + F::cast_from(0.19756347548806534796e1_f64) * t6574 * t3266 - F::cast_from(0.65854491829355115987e0_f64) * t19090 * t3258 * t22007 * t22980;
    t23535
}
