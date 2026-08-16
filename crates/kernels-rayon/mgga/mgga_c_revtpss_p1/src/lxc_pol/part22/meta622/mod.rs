//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta622 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2532;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2533;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2534;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2535;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta622(t1045: f64, t19836: f64, t3117: f64, t11866: f64, t11927: f64, t15716: f64, t15771: f64, t15774: f64, t15776: f64, t15817: f64, t1671: f64, t19819: f64, t19827: f64, t19831: f64, t3115: f64, t4831: f64, t4834: f64, t4869: f64, t4879: f64, t6273: f64, t11134: f64, t11890: f64, t15189: f64, t15874: f64, t15875: f64, t15876: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64, t341: f64, t225: f64, t366: f64, t15696: f64, t4782: f64, t4787: f64, t1058: f64, t6318: f64, t1053: f64, t6317: f64, t4786: f64, t6096: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19837, t19838, t19841) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2532(t1045, t19836, t3117, t11866, t11927, t15716, t15771, t15774, t15776, t15817, t1671, t19819, t19827, t19831, t3115, t4831, t4834, t4869, t4879, t6273);
        let t19855 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2533(t11134, t11890, t15189, t15874, t15875, t15876, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
        let t19856 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2534(t19855, t341);
        let (t19857, t19858, t19861, t19864, t19867, t19869, t19872) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2535(t19856, t225, t366, t15696, t4782, t4787, t1058, t6318, t1053, t6317, t4786, t6096);
    (t19837, t19838, t19841, t19855, t19856, t19857, t19858, t19861, t19864, t19867, t19869, t19872)
}
