//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1706;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1707;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1708;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1709;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta356<F: Float>(t11144: F, t11821: F, t10356: F, t1012: F, t11150: F, t3252: F, t11156: F, t4919: F, t11165: F, t4915: F, t1066: F, t11169: F, t247: F, t1011: F, t1025: F, t1063: F, t11802: F, t11806: F, t11811: F, t11814: F, t11818: F, t3177: F, t3184: F, t3188: F, t3241: F, t3248: F, t3255: F, t4837: F, t283: F, t2857: F, t66: F, t11145: F, t3298: F, t994: F, t4891: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11823, t11824, t11828, t11829, t11836, t11839, t11845) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1706::<F>(t11144, t11821, t10356, t1012, t11150, t3252, t11156, t4919, t11165, t4915, t1066, t11169, t247);
        let t11850 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1707::<F>(t1011, t1025, t1063, t11802, t11806, t11811, t11814, t11818, t11824, t11829, t11836, t11839, t11845, t3177, t3184, t3188, t3241, t3248, t3255, t4837);
        let t11852 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1708::<F>(t283, t2857);
        let (t11853, t11855, t11858, t11859) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1709::<F>(t11852, t66, t11145, t247, t3298, t994, t4891);
    (t11823, t11828, t11845, t11850, t11852, t11853, t11855, t11858, t11859)
}
