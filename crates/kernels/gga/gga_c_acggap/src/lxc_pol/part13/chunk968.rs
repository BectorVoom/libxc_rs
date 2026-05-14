//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 968/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk968<F: Float>(t34957: F, t1181: F, t21143: F, t604: F, t7493: F, t22401: F, t7413: F, t1165: F, t20775: F, t30698: F, t22710: F, t599: F, t4680: F, t7337: F, t8774: F, t4263: F, t7351: F, t7575: F) -> (F, F, F, F, F, F, F, F) {
    let t34958 = 0.28582678745379824648e-3 * t34957;
    let t34961 = t7493 * t1181 * t604 * t21143;
    let t34962 = 0.31448092289604152068e-2 * t34961;
    let t34965 = t7413 * t1181 * t604 * t22401;
    let t34969 = t30698 * t1165 * t604 * t20775;
    let t34973 = t7413 * t1181 * t604 * t22710;
    let t34977 = t30698 * t1181 * t599 * t20775;
    let t34980 = t7337 * t4680 * t8774;
    let t34984 = t7575 * t1181 * t7351 * t4263;
    (t34958, t34962, t34965, t34969, t34973, t34977, t34980, t34984)
}
