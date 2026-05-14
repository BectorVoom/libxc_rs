//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1204/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1204<F: Float>(t2667: F, t6508: F, t2670: F, t6504: F, t20868: F, t924: F, t19890: F, t6085: F, t8081: F, t1616: F, t783: F, t8279: F, t2558: F, t5147: F, t5148: F, t2651: F, t6345: F) -> (F, F, F, F, F, F, F) {
    let t24732 = t2667 * t6508;
    let t24733 = 0.12713391885412927226e1 * t24732;
    let t24734 = t2670 * t6504;
    let t24735 = 0.38140175656238781678e1 * t24734;
    let t24742 = t20868 * t924;
    let t24755 = t6085 * t19890 * t8081;
    let t24756 = 0.2037639021386884617e0 * t24755;
    let t24758 = t783 * t8279 * t1616;
    let t24759 = 0.2037639021386884617e0 * t24758;
    let t24774 = t5147 * t5148 * t2558;
    let t24776 = t2651 * t6345;
    (t24733, t24735, t24742, t24756, t24759, t24774, t24776)
}
