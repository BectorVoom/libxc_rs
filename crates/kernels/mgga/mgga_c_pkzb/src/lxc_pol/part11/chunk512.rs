//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 512/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk512<F: Float>(t1556: F, t1631: F, t1009: F, t496: F, t501: F, t1671: F, t1008: F, t46: F, t552: F, t1555: F, t1596: F, t1604: F, t1627: F, t1629: F, t1641: F, t1669: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2613 = F::cast_from(4.0_f64) * t1556;
    let t2614 = F::cast_from(4.0_f64) * t1631;
    let t2615 = t496 * t1009;
    let t2616 = F::cast_from(4.0_f64) * t2615;
    let t2617 = t501 * t1009;
    let t2618 = F::cast_from(4.0_f64) * t2617;
    let t2619 = F::cast_from(0.18311447306006545054e-3_f64) * t1671;
    let t2620 = t1008 * t46;
    let t2621 = t2620 * t552;
    let t2622 = F::cast_from(0.18311447306006545054e-3_f64) * t2621;
    let t2623 = -t1555 - t2613 + t1627 + t1629 - t2614 + t2616 - t2618 + t1604 + t1641 - t1596 + t1669 - t2619 - t2622;
    (t2613, t2614, t2615, t2616, t2617, t2618, t2619, t2620, t2621, t2622, t2623)
}
