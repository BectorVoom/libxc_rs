//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1444/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1444<F: Float>(t1824: F, t8617: F, t1338: F, t33266: F, t1985: F, t1998: F, t214: F, t27051: F, t1992: F, t550: F, t6976: F, t93505: F) -> (F, F, F, F) {
    let t122471 = t8617 * t1824;
    let t122475 = t1338 * t33266;
    let t122483 = t1985 * t214 * t1998 * t27051;
    let t122488 = t1992 * t6976 * t93505 * t550;
    (t122471, t122475, t122483, t122488)
}
