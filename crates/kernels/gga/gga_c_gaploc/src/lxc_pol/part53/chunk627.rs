//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 627/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk627<F: Float>(t12990: F, t9287: F, t10268: F, t2365: F, t4391: F, t3263: F, t8862: F, t2969: F, t3322: F, t3009: F, t3234: F, t1445: F, t2087: F, t1645: F, t3255: F, t3025: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12991 = t12990 * t9287;
    let t12992 = 0.29792074959875355558e-1 * t12991;
    let t12996 = t2365 * t10268;
    let t12997 = t4391 * t12996;
    let t12998 = 0.59584149919750711116e-1 * t12997;
    let t13004 = 2.0 * t8862 * t3263;
    let t13005 = t2969 * t3322;
    let t13012 = t3009 * t3234;
    let t13013 = t1445 * t13012;
    let t13015 = 0.69017266717057349418e1 * t2087 * t13013;
    let t13016 = t1645 * t3255;
    let t13018 = 0.10725146985555128001e1 * t3025 * t13016;
    (t12992, t12996, t12998, t13004, t13005, t13012, t13013, t13015, t13016, t13018)
}
