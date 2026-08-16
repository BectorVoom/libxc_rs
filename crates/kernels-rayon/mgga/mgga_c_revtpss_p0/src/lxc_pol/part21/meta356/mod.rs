//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1706;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1707;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1708;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta356(t11144: f64, t11821: f64, t10356: f64, t1012: f64, t11150: f64, t3252: f64, t11156: f64, t4919: f64, t11165: f64, t4915: f64, t1066: f64, t11169: f64, t247: f64, t1011: f64, t1025: f64, t1063: f64, t11802: f64, t11806: f64, t11811: f64, t11814: f64, t11818: f64, t3177: f64, t3184: f64, t3188: f64, t3241: f64, t3248: f64, t3255: f64, t4837: f64, t283: f64, t2857: f64, t66: f64, t11145: f64, t3298: f64, t994: f64, t4891: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11823, t11824, t11828, t11829, t11836, t11839, t11845) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1706(t11144, t11821, t10356, t1012, t11150, t3252, t11156, t4919, t11165, t4915, t1066, t11169, t247);
        let t11850 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1707(t1011, t1025, t1063, t11802, t11806, t11811, t11814, t11818, t11824, t11829, t11836, t11839, t11845, t3177, t3184, t3188, t3241, t3248, t3255, t4837);
        let t11852 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1708(t283, t2857);
        let (t11853, t11855, t11858, t11859) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1709(t11852, t66, t11145, t247, t3298, t994, t4891);
    (t11823, t11828, t11845, t11850, t11852, t11853, t11855, t11858, t11859)
}
