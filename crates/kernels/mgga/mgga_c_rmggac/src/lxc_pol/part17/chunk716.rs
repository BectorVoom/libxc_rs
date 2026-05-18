//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 716/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk716<F: Float>(t7788: F, t9705: F, t305: F, t9812: F, t338: F, t9926: F, t118: F, t10154: F, t10156: F, t10158: F, t10162: F, t10164: F, t10168: F, t10170: F, t10174: F, t10177: F, t10179: F, t10181: F, t5266: F, t838: F, t8998: F, t9583: F, t9586: F, t9852: F, t9855: F, t9960: F) -> (F, F) {
    let t10183 = t7788 * t9705;
    let t10185 = t305 * t9812;
    let t10189 = t338 * t9926;
    let t10190 = t118 * t10189;
    let t10193 = -F::new(0.14967802127329760705e-1) * t10154 + F::new(0.2993560425465952141e-1) * t10156 - F::new(0.44903406381989282115e-1) * t10158 - F::new(0.39914139006212695214e-1) * t118 * t9960 - F::new(0.10227998120342003148e-1) * t10162 - F::new(0.6818665413561335432e-1) * t10164 - F::new(0.68186654135613354322e-2) * t10168 - F::new(0.20455996240684006296e-1) * t10170 + F::new(0.23948483403727617128e0) * t838 * t9855 + F::new(0.23948483403727617128e0) * t5266 * t10174 + F::new(0.8980681276397856423e-1) * t10177 - F::new(0.27274661654245341728e-1) * t10179 + F::new(0.81823984962736025184e-1) * t10181 + F::new(0.20455996240684006296e-1) * t10183 + F::new(0.2993560425465952141e-1) * t10185 + F::new(0.11974241701863808564e0) * t118 * t9852 + F::new(0.19957069503106347607e-1) * t10190 + F::new(0.79828278012425390426e-1) * t8998 - t9583 + t9586;
    (t10189, t10193)
}
