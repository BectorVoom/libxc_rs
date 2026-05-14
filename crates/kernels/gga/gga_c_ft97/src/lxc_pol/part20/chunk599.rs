//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 599/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk599<F: Float>(t2428: F, t3780: F, t3751: F, t688: F, t2394: F, t1096: F, t2417: F, t13434: F, t9524: F, t2455: F, t680: F, t1113: F, t695: F, t3758: F, t122: F, t677: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13444 = t3780 * t2428;
    let t13448 = t3751 * t688;
    let t13449 = t2394 * t13448;
    let t13452 = t1096 * t2417;
    let t13453 = t2394 * t13452;
    let t13456 = t9524 * t13434;
    let t13460 = t680 * t1096 * t2455;
    let t13463 = t695 * t1113;
    let t13464 = t3758 * t13463;
    let t13467 = t695 * t122;
    let t13468 = t677 * t13467;
    (t13444, t13448, t13449, t13452, t13453, t13456, t13460, t13464, t13468)
}
