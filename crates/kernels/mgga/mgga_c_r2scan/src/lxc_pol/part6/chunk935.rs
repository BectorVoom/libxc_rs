//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 935/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk935<F: Float>(t374: F, t6772: F, t1353: F, t885: F, t288: F, t5054: F, t97: F, t1543: F, t2267: F, t2858: F, t2337: F, t860: F, t4696: F, t4703: F, t4880: F, t4882: F, t4884: F, t4887: F, t4891: F, t4893: F, t4895: F, t4897: F, t4899: F, t4901: F, t4936: F, t4961: F, t6598: F, t6602: F, t6606: F) -> (F, F, F, F, F, F) {
    let t6773 = t6772 * t374;
    let t6776 = t1353 * t885;
    let t6777 = 3.0 * t6776;
    let t6779 = t97 * t5054 * t288;
    let t6780 = 6.0 * t6779;
    let t6782 = t2858 * t2267 * t1543;
    let t6783 = 18.0 * t6782;
    let t6785 = t860 * t2337;
    let t6786 = 3.0 * t6785;
    let t6787 = -t4696 - t4880 - t4882 - t4884 - t4887 + t4891 - t4893 + t4895 - t4703 + t4897 - t4899 - t4901 - t4936 - t6598 - t6602 - t6606 - t4961;
    (t6773, t6777, t6780, t6783, t6786, t6787)
}
