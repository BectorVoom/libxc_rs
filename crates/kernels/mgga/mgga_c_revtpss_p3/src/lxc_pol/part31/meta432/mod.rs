//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1547;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1548;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta432<F: Float>(t359: F, t6343: F, t999: F, t1086: F, t6235: F, t1647: F, t4995: F, t3153: F, t6299: F, t4983: F, t4998: F, t19482: F, t19501: F, t1089: F, t1678: F, t4866: F, t6271: F, t3298: F, t342: F, t1024: F, t1087: F, t1090: F, t12116: F, t12122: F, t12127: F, t16381: F, t1689: F, t1692: F, t3278: F, t4743: F, t4857: F, t4954: F, t4970: F, t4981: F, t4984: F, t4996: F, t4999: F, t5009: F, t5012: F, t6375: F, t6383: F, t3316: F, t73: F, t4976: F, t1082: F, t19414: F, t1045: F, t3117: F) -> (F, F, F, F, F, F, F, F) {
        let (t19557, t19566, t19569, t19572, t19573, t19576, t19579) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1547::<F>(t359, t6343, t999, t1086, t6235, t1647, t4995, t3153, t6299, t4983, t4998, t19482);
        let t19606 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1548::<F>(t19501, t19579, t1089, t1678, t4866, t3153, t6271, t4983, t4998, t3298, t342, t1024, t1087, t1090, t12116, t12122, t12127, t16381, t1647, t1689, t1692, t19557, t19566, t19569, t19573, t19576, t3278, t4743, t4857, t4954, t4970, t4981, t4984, t4996, t4999, t5009, t5012, t6375, t6383);
        let (t19608, t19611, t19612, t19617, t19622) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1549::<F>(t1678, t3316, t342, t6299, t73, t4976, t1082, t19414, t1045, t999, t6271, t3117);
    (t19572, t19579, t19606, t19608, t19611, t19612, t19617, t19622)
}
