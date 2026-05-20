//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta390 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1835;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1836;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1837;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta390<F: Float>(t12732: F, t1287: F, t487: F, t12646: F, t1280: F, t1269: F, t3588: F, t1204: F, t3781: F, t1214: F, t1209: F, t5462: F, t3601: F, t3769: F, t5477: F, t3783: F, t12690: F, t12699: F, t12702: F, t12706: F, t12709: F, t12714: F, t12717: F, t12719: F, t12723: F, t12727: F, t1285: F, t1288: F, t1291: F, t3552: F, t3670: F, t3746: F, t3755: F, t3756: F, t3770: F, t3774: F, t3784: F, t490: F, t5463: F, t5478: F, t12621: F, t3634: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12734, t12737, t12741, t12744, t12748, t12751) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1835::<F>(t12732, t1287, t487, t12646, t1280, t1269, t3588, t1204, t3781, t1214, t1209, t5462);
        let (t12752, t12753, t12756) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1836::<F>(t1214, t3601, t3769, t1209, t5477);
        let (t12757, t12766) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1837::<F>(t12752, t3783, t12690, t12699, t12702, t12706, t12709, t12714, t12717, t12719, t12723, t12727, t12734, t12737, t12741, t12744, t12748, t12751, t12753, t12756, t1285, t1288, t1291, t3552, t3670, t3746, t3755, t3756, t3770, t3774, t3784, t490, t5463, t5478);
        let (t12769, t12772) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1838::<F>(t12621, t1280, t3634, t828);
    (t12734, t12737, t12741, t12744, t12748, t12751, t12753, t12756, t12757, t12766, t12769, t12772)
}
