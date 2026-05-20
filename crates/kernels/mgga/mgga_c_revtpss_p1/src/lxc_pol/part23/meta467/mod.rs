//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta467 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1911;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1912;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1913;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1914;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta467<F: Float>(t1045: F, t19836: F, t3117: F, t11866: F, t11927: F, t15716: F, t15771: F, t15774: F, t15776: F, t15817: F, t1671: F, t19819: F, t19827: F, t19831: F, t3115: F, t4831: F, t4834: F, t4869: F, t4879: F, t6273: F, t11134: F, t11890: F, t15189: F, t15874: F, t15875: F, t15876: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F, t341: F, t225: F, t366: F, t15696: F, t4782: F, t4787: F, t1058: F, t6318: F, t1053: F, t6317: F, t4786: F, t6096: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19837, t19838, t19841) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1911::<F>(t1045, t19836, t3117, t11866, t11927, t15716, t15771, t15774, t15776, t15817, t1671, t19819, t19827, t19831, t3115, t4831, t4834, t4869, t4879, t6273);
        let t19855 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1912::<F>(t11134, t11890, t15189, t15874, t15875, t15876, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let t19856 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1913::<F>(t19855, t341);
        let (t19857, t19858, t19861, t19864, t19867, t19869, t19872) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1914::<F>(t19856, t225, t366, t15696, t4782, t4787, t1058, t6318, t1053, t6317, t4786, t6096);
    (t19837, t19838, t19841, t19855, t19856, t19857, t19858, t19861, t19864, t19867, t19869, t19872)
}
