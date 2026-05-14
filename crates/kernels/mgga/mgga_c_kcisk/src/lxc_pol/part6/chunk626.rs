//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 626/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk626<F: Float>(t10714: F, t573: F, t10568: F, t10641: F, t1643: F, t4740: F, t4743: F, t586: F, t657: F, t963: F, t397: F, t662: F, t656: F, t1782: F, t4893: F, t7233: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10715 = t573 * t10714;
    let t10738 = 0.93011851851851851854e0 * t10568;
    let t10739 = 0.36514074074074074075e0 * t10641;
    let t10754 = 1.0 / t4740 / t1643;
    let t10755 = t573 * t10754;
    let t10757 = 1.0 / t4743 / t586;
    let t10761 = 0.28842592592592592592e-1 * t10568;
    let t10791 = t963 * t657;
    let t10793 = t397 * t10791 * t662;
    let t10795 = 0.19989765240197019125e-1 * t656 * t10793;
    let t10798 = t4893 * t1782;
    let t10802 = t7233 * t1782;
    (t10715, t10738, t10739, t10755, t10757, t10761, t10791, t10795, t10798, t10802)
}
