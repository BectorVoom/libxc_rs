//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 524/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk524<F: Float>(t1020: F, t192: F, t1535: F, t1555: F, t1596: F, t1604: F, t1627: F, t1629: F, t1641: F, t1669: F, t2613: F, t2614: F, t2616: F, t2618: F, t2619: F, t2622: F, t2711: F, t2714: F, t2718: F, t568: F) -> (F, F) {
    let t2719 = t192 * t1020;
    let t2723 = F::new(3.0) * t1535 * t2714 * t568 + F::new(6.0) * t2718 * t2719 * t568 + F::new(3.0) * t1535 * t2711 - t1555 - t1596 + t1604 + t1627 + t1629 + t1641 + t1669 - t2613 - t2614 + t2616 - t2618 - t2619 - t2622;
    (t2719, t2723)
}
