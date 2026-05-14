//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 965/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk965<F: Float>(t17266: F, t5664: F, t11913: F, t5656: F, t5638: F, t1924: F, t3960: F, t1928: F, t4169: F, t1392: F, t1981: F, t1017: F, t86: F) -> (F, F, F, F, F, F, F, F) {
    let t17267 = t17266 * t5664;
    let t17268 = 0.3684876543209876543e-2 * t17267;
    let t17274 = t11913 * t5656;
    let t17276 = t11913 * t5638;
    let t17277 = 0.14739506172839506172e-2 * t17276;
    let t17287 = t1924 * t3960;
    let t17292 = t4169 * t1928;
    let t17296 = t1392 * t1981;
    let t17298 = t86 * t1017 * t17296;
    (t17267, t17268, t17274, t17276, t17277, t17287, t17292, t17298)
}
