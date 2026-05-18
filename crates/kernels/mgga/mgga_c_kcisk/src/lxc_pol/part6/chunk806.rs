//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 806/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk806<F: Float>(t1769: F, t8833: F, t7219: F, t7253: F, t25: F, t8815: F, t1773: F, t8821: F, t10409: F, t8481: F, t4811: F, t8883: F) -> (F, F, F, F, F, F) {
    let t23814 = t8833 * t1769;
    let t23840 = t7219 * t7253;
    let t23842 = t25 * t8815;
    let t23843 = t1773 * t23842;
    let t23857 = t25 * t8821;
    let t23858 = t1773 * t23857;
    let t23872 = t10409 * t8481;
    let t23874 = t4811 * t8883;
    (t23814, t23840, t23843, t23858, t23872, t23874)
}
