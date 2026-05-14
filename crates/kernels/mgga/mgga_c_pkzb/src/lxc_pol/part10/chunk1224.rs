//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1224/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1224<F: Float>(t2507: F, t1448: F, t3308: F, t2528: F, t12973: F, t1430: F, t12584: F, t1424: F, t1541: F, t78: F, t1429: F, t2499: F, t1436: F, t3333: F, t4810: F, t1425: F, t16111: F, t3314: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23708 = t2507 * t2507;
    let t23711 = t3308 * t1448;
    let t23716 = t2528 * t2528;
    let t23729 = t12973 * t1430;
    let t23732 = t12584 * t1430;
    let t23736 = t1424 * t78 * t1541;
    let t23739 = t2499 * t1429;
    let t23743 = t4810 * t3333 * t1436;
    let t23747 = t16111 * t3314 * t1425;
    (t23708, t23711, t23716, t23729, t23732, t23736, t23739, t23743, t23747)
}
