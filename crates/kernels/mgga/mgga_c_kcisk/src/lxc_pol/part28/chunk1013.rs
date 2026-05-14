//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1013/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1013<F: Float>(t23433: F, t7181: F, t1746: F, t7175: F, t7156: F, t7157: F, t8768: F, t4957: F, t7180: F, t4954: F, t8763: F, t10939: F, t1224: F, t8510: F) -> (F, F, F, F, F, F) {
    let t23434 = t23433 * t7181;
    let t23437 = t1746 * t7175;
    let t23438 = t7156 * t23437;
    let t23443 = t8768 * t7157;
    let t23446 = t4957 * t7175;
    let t23447 = t7180 * t23446;
    let t23450 = t4954 * t8763;
    let t23451 = t23450 * t7181;
    let t23460 = t1224 * t10939 * t8510;
    (t23434, t23438, t23443, t23447, t23451, t23460)
}
