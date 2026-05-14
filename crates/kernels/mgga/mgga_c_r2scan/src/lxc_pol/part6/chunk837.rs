//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 837/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk837<F: Float>(t5264: F, t5342: F, t5406: F, t5849: F, t5887: F, t5914: F, t5941: F, t5988: F, t246: F, t4696: F, t4703: F, t4721: F, t4880: F, t4882: F, t4884: F, t4887: F, t4891: F, t4893: F, t4895: F, t4897: F, t4899: F, t4901: F, t4961: F, t4964: F, t5879: F) -> (F, F) {
    let t5991 = t5264 + t5342 + t5406 + t5849 + t5887 + t5914 + t5941 + t5988;
    let t5997 = -t4696 - t4880 - t4882 - t4884 - t4887 + t4891 - t4893 + t4895 - t4703 + t4897 - t4899 - 0.285764e-1 * t246 * t5879 - t4901 - t4961 - t4721 + t4964;
    (t5991, t5997)
}
