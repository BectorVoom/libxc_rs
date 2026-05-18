//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 780/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk780<F: Float>(t2505: F, t4597: F, t2543: F, t574: F, t2551: F, t979: F, t10879: F, t2637: F, t2013: F, t2630: F, t5477: F, t2634: F) -> (F, F, F, F, F, F) {
    let t18036 = t2505 * t4597;
    let t18089 = t2543 * t574;
    let t18132 = t979 * t2551;
    let t18355 = t10879 * t2637;
    let t18356 = t2013 * t18355;
    let t18406 = t2630 * t5477;
    let t18408 = t2634 * t5477;
    (t18036, t18089, t18132, t18356, t18406, t18408)
}
