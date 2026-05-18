//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1333/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1333<F: Float>(t11369: F, t410: F, t11483: F, t914: F, t2393: F, t10092: F, t10121: F, t10189: F, t10316: F, t10319: F, t10324: F, t10331: F, t10349: F, t10353: F, t10356: F, t11501: F, t11510: F, t11520: F, t11524: F, t11527: F, t133: F, t19090: F, t19106: F, t19115: F, t19271: F, t19302: F, t19305: F, t22007: F, t23472: F, t2363: F, t2370: F, t2439: F, t28493: F, t2970: F, t31989: F, t3207: F, t32078: F, t32277: F, t32288: F, t32324: F, t3259: F, t3260: F, t3266: F, t3270: F, t3923: F, t6455: F, t7832: F, t943: F, t945: F, t946: F) -> F {
    let t32351 = t410 * t11369;
    let t32359 = t914 * t11483;
    let t32366 = t2393 * t32351;
    let t32395 = -F::new(0.39512695097613069591e1) * t19271 * t11510 + F::new(0.65854491829355115987e0) * t2439 * t11520 + F::new(0.39512695097613069591e1) * t19302 * t11501 + F::new(0.39512695097613069592e1) * t2363 * t32324 * t3260 + F::new(0.65854491829355115987e0) * t943 * t31989 * t133 * t945 + F::new(0.13170898365871023197e1) * t2363 * t32351 * t3260 + F::new(0.39512695097613069591e1) * t3259 * t2970 * t2370 * t10189 + F::new(0.65854491829355115987e0) * t32359 * t946 + F::new(0.65854491829355115987e0) * t19305 * t11527 + F::new(0.19756347548806534796e1) * t10356 * t3266 - F::new(0.65854491829355115987e0) * t32366 * t3270 - F::new(0.11853808529283920877e2) * t10319 * t7832 * t10092 - F::new(0.19756347548806534796e1) * t23472 * t3923 + F::new(0.19756347548806534796e1) * t6455 * t32277 * t10353 - F::new(0.39512695097613069592e1) * t10331 * t10349 - F::new(0.65854491829355115987e0) * t19090 * t32288 * t22007 * t3207 + F::new(0.79025390195226139182e1) * t10324 * t10316 - F::new(0.19756347548806534796e1) * t28493 * t11524 + F::new(0.15805078039045227836e2) * t19106 * t32288 * t22007 * t32078 - F::new(0.23707617058567841754e2) * t19115 * t32288 * t22007 * t10121;
    t32395
}
