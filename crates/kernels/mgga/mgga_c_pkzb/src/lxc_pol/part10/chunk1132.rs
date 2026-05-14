//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1132/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1132<F: Float>(t1167: F, t2370: F, t1697: F, t95: F, t452: F, t987: F, t1143: F, t799: F, t1054: F, t633: F, t440: F, t973: F, t1020: F, t1676: F, t1255: F, t951: F) -> (F, F, F, F, F, F, F, F) {
    let t11476 = t2370 * t1167;
    let t11817 = t1697 * t95;
    let t12227 = t987 * t452;
    let t12315 = t1143 * t799;
    let t12508 = t1054 * t633;
    let t12584 = t973 * t440;
    let t12639 = t1676 * t1020;
    let t12845 = t1255 * t951;
    (t11476, t11817, t12227, t12315, t12508, t12584, t12639, t12845)
}
