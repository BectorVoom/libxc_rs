//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1015/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1015<F: Float>(t5818: F, t5821: F, t5930: F, t5932: F, t5934: F, t5936: F, t5940: F, t5945: F, t5950: F, t5952: F, t5955: F, t5959: F, t5986: F, t5834: F, t5963: F, t5966: F, t5968: F, t5970: F, t5972: F, t5975: F, t5976: F, t5978: F, t5982: F, t5985: F) -> (F, F) {
    let t7842 = -0.571528e-1 * t5930 + 4.0 * t5932 + 4.0 * t5934 - t5818 + t5821 + 0.1445810840040231979e-1 * t5936 + t5940 + t5945 - t5950 - 0.20010214504933333333e-2 * t5952 - 0.40020429009866666666e-2 * t5955 + t5959;
    let t7849 = 80.0 * t5986;
    let t7850 = t5963 - t5966 + 0.43374325201206959368e-1 * t5968 - 0.64212977516902094772e0 * t5970 - 0.2602459512072417562e0 * t5972 - t5975 + 16.0 * t5976 - 0.2258170631111111111e-2 * t5978 + t5834 - 40.0 * t5982 + t5985 - t7849;
    (t7842, t7850)
}
