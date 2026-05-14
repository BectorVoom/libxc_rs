//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1190/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1190<F: Float>(t1: F, t11473: F, t14640: F, t16975: F, t25969: F, t2601: F, t2633: F, t2668: F, t2721: F, t2722: F, t2758: F, t2812: F, t2813: F, t297: F, t313: F, t33398: F, t3917: F, t42129: F, t42145: F, t42991: F, t43003: F, t43112: F, t52111: F, t52138: F, t52154: F, t56704: F, t56727: F, t56744: F, t57554: F, t57670: F, t57678: F, t57701: F, t57857: F, t57938: F, t7482: F, t914: F, t930: F) -> (F,) {
    let t57988 = 0.11590881986385010473e0 * t930 * t914 * t56704 - 0.23229342182245570105e2 * t2758 * t313 * t56744 * t1 * t297 + 0.12388982497197637389e3 * t52111 + 0.23442632909977165232e4 * t3917 * t42129 * t56727 - 0.51620760404990155789e2 * t2668 * t42145 * t16975 + 0.93568771831764348721e2 * t2812 * t2813 * t57678 + 0.15454509315180013964e0 * t930 * t914 * t2633 * t57857 + 0.15146801702008125515e1 * t2721 * t2722 * t57938 + 0.17581974682482873924e4 * t3917 * t14640 * t57554 - 0.15146801702008125515e1 * t42991 - 0.10097867801338750343e1 * t43003 - t25969 + 0.58606582274942913081e3 * t43112 + 0.779739765264702906e2 * t11473 * t7482 * t57701 + 0.1559479530529405812e2 * t2812 * t2813 * t57670 - 0.23181763972770020945e0 * t930 * t914 * t2601 * t57857 + 0.20195735602677500687e1 * t52138 + 0.15454509315180013964e0 * t52154 + 0.69310201356862480534e1 * t33398;
    (t57988,)
}
