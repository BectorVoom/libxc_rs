//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 735/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk735<F: Float>(t7450: F, t8577: F, t1609: F, t1986: F, t7720: F, t1212: F, t1970: F, t1971: F, t209: F, t511: F, t558: F, t515: F, t570: F, t7244: F, t8447: F, t321: F) -> (F, F, F, F, F, F) {
    let t38395 = t8577 * t7450;
    let t38397 = t1986 * t1609;
    let t38398 = t7720 * t38397;
    let t38404 = t1970 * t1971 * t511 * t558 * t1212 * t209;
    let t38412 = t1970 * t1971 * t515 * t570 * t1212 * t209;
    let t38414 = t7244 * t8447;
    let t38416 = t209 * t321;
    (t38395, t38398, t38404, t38412, t38414, t38416)
}
