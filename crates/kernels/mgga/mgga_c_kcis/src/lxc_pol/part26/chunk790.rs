//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 790/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk790<F: Float>(t5857: F, t738: F, t5860: F, t1441: F, t1951: F, t1962: F, t4016: F, t1014: F, t5872: F, t1928: F, t4161: F, t2820: F, t5659: F, t86: F, t5664: F, t11913: F, t5656: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17237 = t738 * t5857;
    let t17240 = 0.17611111111111111111e-2 * t738 * t5860;
    let t17248 = t1441 * t1951;
    let t17250 = t4016 * t1962;
    let t17259 = t1014 * t5872;
    let t17260 = 0.33163888888888888888e-2 * t17259;
    let t17261 = t4161 * t1928;
    let t17266 = t86 * t2820 * t5659;
    let t17267 = t17266 * t5664;
    let t17268 = 0.3684876543209876543e-2 * t17267;
    let t17274 = t11913 * t5656;
    (t17237, t17240, t17248, t17250, t17259, t17260, t17261, t17267, t17268, t17274)
}
