//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 867/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk867<F: Float>(t1042: F, t23822: F, t11632: F, t23641: F, t11250: F, t1668: F, t6244: F, t1045: F, t3117: F, t1469: F, t5825: F, t4806: F, t23633: F, t4801: F, t1651: F, t4872: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23823 = t1042 * t23822;
    let t23829 = t23641 * t11632;
    let t23830 = t1042 * t23829;
    let t23833 = t23641 * t11250;
    let t23834 = t1042 * t23833;
    let t23837 = t6244 * t1668;
    let t23838 = t23837 * t1045;
    let t23839 = t3117 * t23838;
    let t23842 = t5825 * t1469;
    let t23843 = t4806 * t23842;
    let t23844 = t1042 * t23843;
    let t23847 = t4806 * t23633;
    let t23848 = t1042 * t23847;
    let t23851 = t4801 * t23842;
    let t23852 = t1042 * t23851;
    let t23857 = t5825 * t1651;
    let t23858 = t4872 * t23857;
    (t23823, t23830, t23834, t23837, t23839, t23842, t23844, t23848, t23852, t23858)
}
