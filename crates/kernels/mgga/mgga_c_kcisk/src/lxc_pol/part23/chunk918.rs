//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 918/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk918<F: Float>(t18984: F, t18985: F, t3482: F, t1163: F, t5996: F, t13377: F, t6001: F, t1341: F, t14264: F, t5991: F, t1286: F, t5967: F, t1450: F, t1415: F, t1411: F, t14188: F, t5992: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18986 = t18984 * t18985;
    let t18987 = t3482 * t18986;
    let t18989 = t5996 * t1163;
    let t18990 = t13377 * t18989;
    let t18991 = t3482 * t18990;
    let t18993 = t6001 * t1163;
    let t18994 = t13377 * t18993;
    let t18995 = t3482 * t18994;
    let t18997 = t14264 * t1341;
    let t18998 = t5991 * t1163;
    let t18999 = t18997 * t18998;
    let t19000 = t3482 * t18999;
    let t19005 = t5967 * t1286;
    let t19006 = t1450 * t19005;
    let t19007 = t1415 * t19006;
    let t19008 = t1411 * t19007;
    let t19010 = t14188 * t5992;
    (t18987, t18989, t18991, t18993, t18995, t18998, t19000, t19005, t19008, t19010)
}
