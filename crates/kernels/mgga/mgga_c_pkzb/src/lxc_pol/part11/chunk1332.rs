//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1332/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1332<F: Float>(t1255: F, t3928: F, t1245: F, t3874: F, t11345: F, t410: F, t1227: F, t3903: F, t3880: F, t6517: F, t2393: F, t10070: F, t10310: F, t10311: F, t10320: F, t10341: F, t10344: F, t10349: F, t10352: F, t11506: F, t11507: F, t11524: F, t1250: F, t19078: F, t22007: F, t23465: F, t2363: F, t26927: F, t26948: F, t28457: F, t31827: F, t3187: F, t31920: F, t32225: F, t3260: F, t3270: F, t3273: F, t3914: F, t3920: F, t394: F, t397: F, t6514: F, t6523: F, t7832: F, t8507: F, t8512: F, t8516: F, t8519: F, t8549: F, t919: F) -> (F, F, F, F, F) {
    let t32261 = t1255 * t3928;
    let t32277 = t1245 * t3874;
    let t32288 = t410 * t11345;
    let t32293 = t3903 * t1227;
    let t32297 = t6517 * t3880;
    let t32324 = t1245 * t3880;
    let t32325 = t2393 * t32324;
    let t32337 = F::new(0.11853808529283920877e2) * t6514 * t32277 * t10311 + F::new(0.19756347548806534796e1) * t8549 * t3920 + F::new(0.19756347548806534796e1) * t26927 * t1250 + F::new(0.19756347548806534796e1) * t10352 * t7832 * t10070 + F::new(0.92196288561097162379e1) * t19078 * t32288 * t22007 * t3187 - F::new(0.19756347548806534796e1) * t2393 * t32293 * t3270 + F::new(0.11853808529283920877e2) * t8507 * t7832 * t32297 * t919 - F::new(0.11853808529283920877e2) * t8519 * t7832 * t11506 * t919 - F::new(0.19756347548806534796e1) * t28457 * t11524 + F::new(0.39512695097613069592e1) * t2363 * t32293 * t3260 + F::new(0.39512695097613069591e1) * t8516 * t11507 + F::new(0.19756347548806534796e1) * t3273 * t10341 + F::new(0.39512695097613069591e1) * t8512 * t11507 + F::new(0.19756347548806534796e1) * t26948 * t7832 * t31920 * t394 - F::new(0.11853808529283920877e2) * t6523 * t32277 * t10320 - F::new(0.19756347548806534796e1) * t32325 * t3270 + F::new(0.39512695097613069591e1) * t23465 * t3914 + F::new(0.11853808529283920877e2) * t10310 * t7832 * t31827 - F::new(0.19756347548806534796e1) * t10344 * t10349 + F::new(0.65854491829355115987e0) * t397 * t32225;
    (t32261, t32277, t32288, t32324, t32337)
}
