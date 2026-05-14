//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 641/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk641<F: Float>(t14275: F, t14288: F, t209: F, t1016: F, t12032: F, t2798: F, t3718: F, t1382: F, t2854: F, t3689: F, t1445: F, t2778: F, t14271: F, t12054: F, t12881: F, t13354: F, t13356: F, t13365: F, t13370: F, t13374: F, t13775: F, t13776: F, t1562: F, t2877: F, t3702: F, t574: F, t597: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14289 = t14275 + t14288;
    let t14290 = t14289 * t209;
    let t14292 = 2.0 * t12032 * t1016;
    let t14294 = 2.0 * t2798 * t3718;
    let t14295 = t1016 * t3718;
    let t14297 = 4.0 * t1382 * t14295;
    let t14298 = t2854 * t3689;
    let t14299 = t1445 * t14298;
    let t14302 = t2778 * t3689;
    let t14303 = t1445 * t14302;
    let t14306 = t1445 * t14271;
    let t14313 = t13354 + t13356 + t13365 - t13775 + t13776 - t13370 - t13374 - 0.13803453343411469884e2 * t1562 * t14299 - 0.92023022289409799224e1 * t574 * t14303 + 0.23005755572352449806e2 * t597 * t14306 + 0.71500979903700853338e0 * t3702 * t2877 - 0.21450293971110256002e1 * t12054 * t12881;
    (t14289, t14290, t14292, t14294, t14295, t14297, t14298, t14299, t14302, t14303, t14306, t14313)
}
