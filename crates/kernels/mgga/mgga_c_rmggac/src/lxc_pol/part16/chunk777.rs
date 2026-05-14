//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 777/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk777<F: Float>(t39285: F, t39295: F, t39338: F, t39405: F, t39451: F, t39528: F, t39544: F, t39591: F, t2265: F, t5026: F, t39667: F, t39678: F, t39698: F, t39701: F, t39785: F, t39796: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42906 = 0.39726959900411316772e-4 * t39285;
    let t42909 = 0.39726959900411316772e-4 * t39295;
    let t42928 = 0.60975299583150056624e-3 * t39338;
    let t42954 = 0.39726959900411316772e-4 * t39405;
    let t42970 = 0.3193131120497015617e0 * t39451;
    let t43001 = 0.3193131120497015617e0 * t39528;
    let t43008 = 0.47896966807455234256e0 * t39544;
    let t43042 = 0.1489760996265424379e-3 * t39591;
    let t43043 = t5026 * t2265;
    let t43096 = 0.10909864661698136692e0 * t39667;
    let t43100 = 0.15965655602485078085e0 * t39678;
    let t43107 = 0.10909864661698136692e0 * t39698;
    let t43108 = 0.47896966807455234256e0 * t39701;
    let t43135 = 0.60975299583150056624e-3 * t39785;
    let t43138 = 0.60975299583150056624e-3 * t39796;
    (t42906, t42909, t42928, t42954, t42970, t43001, t43008, t43042, t43043, t43096, t43100, t43107, t43108, t43135, t43138)
}
