//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1249;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1250;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1251;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1252;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1253;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta379(t19127: f64, t935: f64, t915: f64, t11294: f64, t6145: f64, t11465: f64, t6189: f64, t4733: f64, t981: f64, t11108: f64, t6400: f64, t1100: f64, t18902: f64, t19025: f64, t19027: f64, t19029: f64, t19031: f64, t19048: f64, t19051: f64, t19053: f64, t19055: f64, t19058: f64, t19060: f64, t19062: f64, t19079: f64, t19081: f64, t19084: f64, t5023: f64, t3022: f64, t6219: f64, t6223: f64, t2986: f64, t6205: f64, t974: f64, t4708: f64, t4724: f64, t3336: f64, t6396: f64, t6184: f64, t964: f64, t19021: f64, t973: f64, t11461: f64, t11554: f64, t15343: f64, t1634: f64, t2982: f64, t4685: f64, t6190: f64, t6206: f64, t6209: f64, t965: f64, t6152: f64, t945: f64, t15170: f64, t15189: f64, t15312: f64, t15322: f64, t15324: f64, t18944: f64, t18961: f64, t18964: f64, t18967: f64, t18970: f64, t18973: f64, t11134: f64, t11366: f64, t11422: f64, t11423: f64, t18948: f64, t19002: f64, t19004: f64, t19007: f64, t19009: f64, t19014: f64, t19017: f64, t15123: f64, t15125: f64, t15301: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18951: f64, t18980: f64, t18982: f64, t18985: f64, t18988: f64, t18990: f64, t18993: f64, t18995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19130, t19132, t19136, t19141) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1249(t19127, t935, t915, t11294, t6145, t11465, t6189, t4733, t981, t11108, t6400, t1100, t18902, t19025, t19027, t19029, t19031, t19048, t19051, t19053, t19055, t19058, t19060, t19062, t19079, t19081, t19084, t5023);
        let (t19143, t19145, t19149, t19152, t19153, t19156) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1250(t3022, t6219, t6223, t2986, t6205, t974, t981, t4708, t4724, t3336, t6396, t6184, t964);
        let t19172 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1251(t19021, t973, t11461, t11554, t15343, t1634, t19029, t19031, t19058, t19060, t19062, t19156, t2982, t4685, t4708, t6190, t6206, t6209, t965, t974);
        let (t19173, t19202) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1252(t6152, t945, t15170, t15189, t15312, t15322, t15324, t18944, t18961, t18964, t18967, t18970, t18973);
        let t19226 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1253(t11134, t11366, t11422, t11423, t18948, t19002, t19004, t19007, t19009, t19014, t19017, t15123, t15125, t15301, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18951, t18980, t18982, t18985, t18988, t18990, t18993, t18995, t19202);
    (t19130, t19132, t19136, t19141, t19143, t19145, t19149, t19152, t19153, t19172, t19173, t19226)
}
