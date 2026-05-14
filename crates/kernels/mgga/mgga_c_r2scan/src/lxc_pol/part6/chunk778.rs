//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 778/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk778<F: Float>(t423: F, t5249: F, t170: F, t1727: F, t597: F, t1375: F, t1859: F, t1862: F, t1823: F, t732: F, t1818: F, t712: F, t1822: F, t234: F, t5202: F, t5205: F, t5209: F, t5212: F, t5213: F, t5218: F, t5220: F, t5225: F, t5230: F, t5233: F, t5235: F, t5237: F, t5248: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5250 = t5249 * t423;
    let t5251 = t170 * t1727;
    let t5252 = t597 * t5251;
    let t5253 = t5250 * t5252;
    let t5255 = t1859 * t1375;
    let t5256 = t5255 * t1862;
    let t5258 = t732 * t1823;
    let t5260 = t1818 * t712;
    let t5261 = t5260 * t1822;
    let t5263 = 0.30762056574649219973e4 * t234 * t5261;
    let t5264 = -t5202 - t5205 - t5209 + t5212 + 0.80040858019733333331e-2 * t5213 - t5218 - 0.1016176784e-1 * t5220 - t5225 + t5230 - t5233 - 0.16265371950452609763e-1 * t5235 - 0.32530743900905219526e-1 * t5237 + t5248 - 0.12154685976e1 * t5253 + 0.4051561992e0 * t5256 + 0.30762056574649219973e4 * t5258 + t5263;
    (t5250, t5251, t5252, t5253, t5255, t5256, t5258, t5260, t5261, t5263, t5264)
}
