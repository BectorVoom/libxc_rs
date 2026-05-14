//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 888/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk888<F: Float>(t2075: F, t8048: F, t13472: F, t1313: F, t30158: F, t1312: F, t6205: F, t8054: F, t6204: F, t1309: F, t20128: F, t20255: F, t2164: F, t25981: F, t25985: F, t26008: F, t26065: F, t26075: F, t26086: F, t30270: F, t30274: F, t30278: F, t30505: F, t3935: F, t405: F, t6157: F, t8033: F, t8037: F, t8041: F, t8045: F) -> (F,) {
    let t30510 = t2075 * t8048;
    let t30511 = t13472 * t30510;
    let t30514 = t1313 * t30158;
    let t30515 = t1312 * t30514;
    let t30522 = t6205 * t8054;
    let t30523 = t6204 * t30522;
    let t30534 = 0.10794473229706390328e0 * t3935 * t30270 - 0.10794473229706390328e0 * t3935 * t30274 - 0.53972366148531951639e-1 * t3935 * t30278 + 0.17990788716177317213e-1 * t25981 + 0.35981577432354634425e-1 * t25985 + 0.5397236614853195164e-1 * t30505 * t405 - 0.10794473229706390328e0 * t20255 * t8037 + 0.10794473229706390328e0 * t3935 * t30511 + 0.17990788716177317213e-1 * t1309 * t30515 - 0.10794473229706390328e0 * t6157 * t8041 + 0.53972366148531951639e-1 * t26008 * t2164 + 0.32383419689119170984e0 * t1309 * t30523 + 0.53972366148531951639e-1 * t6157 * t8045 + 0.71963154864709268852e-1 * t6157 * t8033 - 0.35981577432354634425e-1 * t26065 - 0.11993859144118211475e-1 * t20128 - 0.35981577432354634425e-1 * t26075 + 0.2398771828823642295e-1 * t26086;
    (t30534,)
}
