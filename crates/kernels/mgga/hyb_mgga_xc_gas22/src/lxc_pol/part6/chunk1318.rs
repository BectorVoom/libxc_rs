//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1318/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1318<F: Float>(t11478: F, t9681: F, t2893: F, t4501: F, t2851: F, t2889: F, t2858: F, t14770: F, t26118: F, t2829: F, t2834: F, t2838: F, t30790: F, t30793: F, t30796: F, t30807: F, t30826: F, t30829: F, t3661: F, t3688: F, t7637: F, t7806: F, t7811: F, t9549: F, t9625: F, t9766: F) -> (F, F, F, F, F) {
    let t30871 = t11478 * t9681;
    let t30880 = t4501 * t2893;
    let t30881 = t2851 * t30880;
    let t30891 = t4501 * t2889;
    let t30892 = t2858 * t30891;
    let t30895 = t2858 * t30880;
    let t30902 = 32.0 / 9.0 * t7811 * t30871 - 1600.0 / 3.0 * t9549 * t30790 + 1600.0 / 3.0 * t9549 * t30793 - 352.0 / 3.0 * t7806 * t30796 + 64.0 / 81.0 * t3661 * t30881 + 32.0 / 27.0 * t2829 * t30807 + 2520.0 * t26118 * t9625 * t9766 + 64.0 / 27.0 * t3688 * t30881 - 32.0 / 9.0 * t2834 * t30892 + 32.0 / 9.0 * t2838 * t30895 - 448.0 / 27.0 * t14770 * t30826 - 224.0 / 9.0 * t7637 * t30829;
    (t30871, t30891, t30892, t30895, t30902)
}
