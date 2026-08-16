//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1163/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1163<F: Float>(t6330: F, t6890: F, t6889: F, t22685: F, t26193: F, t7700: F, t1985: F, t225: F, t567: F, t6434: F, t214: F, t6460: F, t6906: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28191 = t6890 * t6330;
    let t28192 = t6889 * t28191;
    let t28193 = t22685 * t28192;
    let t28195 = t26193 * t7700;
    let t28196 = t1985 * t28195;
    let t28199 = t6434 * t225 * t567;
    let t28200 = t214 * t28199;
    let t28201 = t1985 * t28200;
    let t28205 = t6906 * t6460;
    (t28191, t28192, t28193, t28195, t28196, t28199, t28200, t28201, t28205)
}
