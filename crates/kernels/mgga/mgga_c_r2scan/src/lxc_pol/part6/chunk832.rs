//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 832/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk832<F: Float>(t1789: F, t406: F, t410: F, t1748: F, t1751: F, t1398: F, t745: F, t735: F, t5770: F, t5774: F, t5777: F, t5793: F, t5812: F, t5815: F, t5818: F, t5821: F, t5919: F, t5920: F, t5923: F, t5925: F, t5927: F, t5930: F) -> (F, F, F, F, F, F) {
    let t5932 = t406 * t1789;
    let t5934 = t410 * t1789;
    let t5936 = t1751 * t1748;
    let t5938 = t1398 * t745;
    let t5940 = 0.21687162600603479684e-1 * t735 * t5938;
    let t5941 = t5770 - t5774 + t5919 - 0.300153217574e-2 * t5920 - t5777 - t5793 + t5923 + t5812 + t5815 + t5925 - 3.0 * t5927 - 0.1714584e0 * t5930 - 12.0 * t5932 + 12.0 * t5934 - t5818 + t5821 + 0.21687162600603479684e-1 * t5936 + t5940;
    (t5932, t5934, t5936, t5938, t5940, t5941)
}
