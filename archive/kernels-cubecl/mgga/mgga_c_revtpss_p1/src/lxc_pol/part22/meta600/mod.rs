//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2487;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2488;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2489;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2490;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta600<F: Float>(t6152: F, t945: F, t15170: F, t15189: F, t15312: F, t15322: F, t15324: F, t18944: F, t18961: F, t18964: F, t18967: F, t18970: F, t18973: F, t11134: F, t11366: F, t11422: F, t11423: F, t18948: F, t19002: F, t19004: F, t19007: F, t19009: F, t19014: F, t19017: F, t15123: F, t15125: F, t15301: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18951: F, t18980: F, t18982: F, t18985: F, t18988: F, t18990: F, t18993: F, t18995: F, t954: F, t11574: F, t15127: F, t15363: F, t15364: F, t4631: F, t4635: F, t2924: F, t11404: F, t11548: F, t15400: F, t1622: F, t19046: F, t19079: F, t19130: F, t19132: F, t2938: F, t311: F, t4647: F, t4670: F, t6158: F, t6174: F, t6177: F, t946: F, t955: F) -> (F, F, F, F, F, F, F) {
        let (t19173, t19202) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2487::<F>(t6152, t945, t15170, t15189, t15312, t15322, t15324, t18944, t18961, t18964, t18967, t18970, t18973);
        let t19226 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2488::<F>(t11134, t11366, t11422, t11423, t18948, t19002, t19004, t19007, t19009, t19014, t19017, t15123, t15125, t15301, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18951, t18980, t18982, t18985, t18988, t18990, t18993, t18995, t19202);
        let (t19227, t19247) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2489::<F>(t19226, t954, t11134, t11574, t15127, t15189, t15363, t15364, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let (t19250, t19252, t19253) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2490::<F>(t4631, t4635, t2924, t11404, t11548, t15400, t1622, t19046, t19079, t19130, t19132, t19173, t19227, t19247, t2938, t311, t4647, t4670, t6158, t6174, t6177, t946, t955);
    (t19173, t19226, t19227, t19247, t19250, t19252, t19253)
}
