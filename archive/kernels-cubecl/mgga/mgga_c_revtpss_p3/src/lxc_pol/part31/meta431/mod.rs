//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1543;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1544;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1545;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta431<F: Float>(t19450: F, t19491: F, t1043: F, t6258: F, t1089: F, t3153: F, t6305: F, t4982: F, t999: F, t1024: F, t1083: F, t1087: F, t11940: F, t12122: F, t12149: F, t16544: F, t16559: F, t16566: F, t19438: F, t19443: F, t19447: F, t19453: F, t19457: F, t19463: F, t19479: F, t19484: F, t19488: F, t3223: F, t3287: F, t4857: F, t4954: F, t4977: F, t4988: F, t4992: F, t4996: F, t5005: F, t6368: F, t4757: F, t5004: F, t3291: F, t6244: F, t1082: F, t19399: F, t4866: F, t4893: F, t1647: F, t4980: F, t1071: F, t3318: F, t3304: F, t16553: F, t1093: F, t11788: F, t12160: F, t15655: F, t16502: F, t16552: F, t1685: F, t3204: F, t3299: F, t3317: F, t4964: F, t4967: F, t4981: F, t4984: F, t6235: F, t6362: F, t6371: F, t6386: F) -> (F, F, F, F, F, F, F) {
        let (t19492, t19497, t19498, t19501) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1543::<F>(t19450, t19491, t1043, t6258, t1089, t3153, t6305);
        let (t19502, t19508) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1544::<F>(t4982, t999, t19501, t1024, t1083, t1087, t11940, t12122, t12149, t16544, t16559, t16566, t19438, t19443, t19447, t19453, t19457, t19463, t19479, t19484, t19488, t19492, t19498, t3223, t3287, t4857, t4954, t4977, t4988, t4992, t4996, t5005, t6368);
        let (t19509, t19512, t19515, t19520, t19521, t19526, t19533) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1545::<F>(t4757, t5004, t3291, t6244, t1082, t19399, t4866, t4982, t4893, t1647, t4980, t1071, t6305);
        let (t19548, t19554) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1546::<F>(t19533, t3318, t3304, t1043, t16553, t19450, t1093, t11788, t12160, t15655, t16502, t16544, t16552, t1685, t19509, t19512, t19515, t19521, t19526, t3204, t3223, t3299, t3317, t4857, t4964, t4967, t4977, t4981, t4984, t6235, t6362, t6371, t6386);
    (t19497, t19501, t19502, t19508, t19520, t19548, t19554)
}
