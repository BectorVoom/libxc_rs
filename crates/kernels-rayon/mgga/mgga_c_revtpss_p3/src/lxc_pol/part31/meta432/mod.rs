//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1547;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1548;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta432(t359: f64, t6343: f64, t999: f64, t1086: f64, t6235: f64, t1647: f64, t4995: f64, t3153: f64, t6299: f64, t4983: f64, t4998: f64, t19482: f64, t19501: f64, t1089: f64, t1678: f64, t4866: f64, t6271: f64, t3298: f64, t342: f64, t1024: f64, t1087: f64, t1090: f64, t12116: f64, t12122: f64, t12127: f64, t16381: f64, t1689: f64, t1692: f64, t3278: f64, t4743: f64, t4857: f64, t4954: f64, t4970: f64, t4981: f64, t4984: f64, t4996: f64, t4999: f64, t5009: f64, t5012: f64, t6375: f64, t6383: f64, t3316: f64, t73: f64, t4976: f64, t1082: f64, t19414: f64, t1045: f64, t3117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19557, t19566, t19569, t19572, t19573, t19576, t19579) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1547(t359, t6343, t999, t1086, t6235, t1647, t4995, t3153, t6299, t4983, t4998, t19482);
        let t19606 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1548(t19501, t19579, t1089, t1678, t4866, t3153, t6271, t4983, t4998, t3298, t342, t1024, t1087, t1090, t12116, t12122, t12127, t16381, t1647, t1689, t1692, t19557, t19566, t19569, t19573, t19576, t3278, t4743, t4857, t4954, t4970, t4981, t4984, t4996, t4999, t5009, t5012, t6375, t6383);
        let (t19608, t19611, t19612, t19617, t19622) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1549(t1678, t3316, t342, t6299, t73, t4976, t1082, t19414, t1045, t999, t6271, t3117);
    (t19572, t19579, t19606, t19608, t19611, t19612, t19617, t19622)
}
