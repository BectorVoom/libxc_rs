//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta427 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1526;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1527;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1528;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1529;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta427<F: Float>(t19045: F, t324: F, t300: F, t6184: F, t983: F, t15547: F, t1642: F, t4719: F, t4725: F, t6104: F, t914: F, t936: F, t15416: F, t1610: F, t4590: F, t4632: F, t11134: F, t11534: F, t15127: F, t15189: F, t15503: F, t15504: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F, t291: F, t4734: F, t6226: F, t974: F, t981: F, t15170: F, t15447: F, t15457: F, t15459: F, t18961: F, t18964: F, t18967: F, t18970: F, t18973: F, t11334: F, t11338: F, t11366: F, t19002: F, t19004: F, t19007: F, t19009: F, t19014: F, t19017: F, t15123: F, t15435: F, t18951: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19046, t19048, t19051, t19053, t19055, t19058) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1526::<F>(t19045, t324, t300, t6184, t983, t15547, t1642, t4719, t4725, t6104, t914, t936);
        let (t19060, t19062, t19077) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1527::<F>(t15416, t1610, t4590, t4632, t11134, t11534, t15127, t15189, t15503, t15504, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19079, t19081, t19084, t19103) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1528::<F>(t19077, t291, t4719, t4734, t6226, t974, t981, t15170, t15189, t15447, t15457, t15459, t18944, t18961, t18964, t18967, t18970, t18973);
        let t19127 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1529::<F>(t11134, t11334, t11338, t11366, t18948, t19002, t19004, t19007, t19009, t19014, t19017, t15123, t15127, t15435, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18951, t18980, t18982, t18985, t18988, t18990, t18993, t18995, t19103);
    (t19046, t19048, t19051, t19053, t19055, t19058, t19060, t19062, t19079, t19081, t19084, t19127)
}
