//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 707/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk707<F: Float>(t1775: F, t4523: F, t4512: F, t363: F, t4606: F, t11756: F, t1800: F, t358: F, t432: F, t11762: F, t11718: F, t11720: F, t11732: F, t11734: F, t11745: F, t11755: F, t11761: F, t16418: F, t16421: F, t16424: F, t16427: F, t16430: F, t16433: F, t16439: F, t16442: F, t3139: F, t462: F, t8301: F, t8302: F) -> (F,) {
    let t16444 = t1775 * t4523;
    let t16446 = t1775 * t4512;
    let t16448 = t4606 * t363;
    let t16449 = t11756 * t16448;
    let t16452 = t1800 * t358;
    let t16454 = t16452 * t4606 * t432;
    let t16457 = t11762 * t16448;
    let t16461 = 8.0 / 3.0 * t3139 * t16418 + 4.0 / 3.0 * t462 * t16421 - 10.0 / 27.0 * t462 * t16424 - 8.0 / 9.0 * t3139 * t16427 + 2.0 / 3.0 * t462 * t16430 + 2.0 / 9.0 * t462 * t16433 + 4.0 / 9.0 * t11718 - 8.0 / 27.0 * t11720 - t11732 + 4.0 * t462 * t16439 - 2.0 / 9.0 * t16442 + t16444 / 9.0 + 2.0 / 27.0 * t16446 + 4.0 / 9.0 * t11755 * t16449 - 4.0 / 3.0 * t11761 * t16454 - 4.0 / 3.0 * t11761 * t16457 - t8301 - t11734 - t11745 - 4.0 / 9.0 * t8302;
    (t16461,)
}
