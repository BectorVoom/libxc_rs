//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 998/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk998<F: Float>(t1411: F, t26830: F, t1450: F, t26416: F, t1340: F, t13959: F, t8172: F, t13306: F, t13400: F, t19757: F, t19760: F, t19762: F, t19788: F, t19972: F, t2174: F, t26799: F, t26803: F, t26807: F, t26811: F, t26816: F, t26820: F, t26825: F, t26828: F, t6218: F, t6221: F) -> (F, F, F, F, F, F) {
    let t26831 = t1411 * t26830;
    let t26833 = t1450 * t26416;
    let t26834 = t1340 * t26833;
    let t26835 = t1411 * t26834;
    let t26841 = t13959 * t8172;
    let t26843 = t13306 * t8172;
    let t26846 = -0.58958024691358024688e-2 * t19757 - t19760 - 0.16581944444444444444e-2 * t26799 - 0.55273148148148148147e-3 * t26803 + 0.88437037037037037035e-2 * t26807 - 0.22109259259259259259e-2 * t26811 + 0.66327777777777777776e-2 * t26816 - 0.55273148148148148147e-2 * t26820 + 0.44218518518518518516e-2 * t26825 + t19762 - 0.49745833333333333332e-2 * t26828 + 0.16581944444444444444e-2 * t26831 + 0.16581944444444444444e-2 * t26835 + t13400 - 0.386e0 * t6221 * t6218 - 0.386e0 * t19972 * t2174 - 0.22109259259259259259e-2 * t26841 - 0.33163888888888888888e-2 * t26843 - 0.11054629629629629629e-2 * t19788;
    (t26831, t26833, t26835, t26841, t26843, t26846)
}
