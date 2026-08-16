//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta443 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1860;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1861;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1862;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1863;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta443<F: Float>(t3022: F, t6219: F, t6223: F, t2986: F, t6205: F, t974: F, t981: F, t4708: F, t4724: F, t3336: F, t6396: F, t6184: F, t964: F, t19021: F, t973: F, t11461: F, t11554: F, t15343: F, t1634: F, t19029: F, t19031: F, t19058: F, t19060: F, t19062: F, t2982: F, t4685: F, t6190: F, t6206: F, t6209: F, t965: F, t6152: F, t945: F, t15170: F, t15189: F, t15312: F, t15322: F, t15324: F, t18944: F, t18961: F, t18964: F, t18967: F, t18970: F, t18973: F, t11134: F, t11366: F, t11422: F, t11423: F, t18948: F, t19002: F, t19004: F, t19007: F, t19009: F, t19014: F, t19017: F, t15123: F, t15125: F, t15301: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19143, t19145, t19147, t19149, t19150, t19152, t19153, t19156) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1860::<F>(t3022, t6219, t6223, t2986, t6205, t974, t981, t4708, t4724, t3336, t6396, t6184, t964);
        let (t19167, t19172) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1861::<F>(t19021, t973, t11461, t11554, t15343, t1634, t19029, t19031, t19058, t19060, t19062, t19156, t2982, t4685, t4708, t6190, t6206, t6209, t965, t974);
        let (t19173, t19202) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1862::<F>(t6152, t945, t15170, t15189, t15312, t15322, t15324, t18944, t18961, t18964, t18967, t18970, t18973);
        let t19226 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1863::<F>(t11134, t11366, t11422, t11423, t18948, t19002, t19004, t19007, t19009, t19014, t19017, t15123, t15125, t15301, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18951, t18980, t18982, t18985, t18988, t18990, t18993, t18995, t19202);
    (t19143, t19145, t19147, t19149, t19150, t19152, t19153, t19156, t19167, t19172, t19173, t19226)
}
