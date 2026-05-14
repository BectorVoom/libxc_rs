//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1369/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1369<F: Float>(t17182: F, t34423: F, t9740: F, t34428: F, t10005: F, t33276: F, t10004: F, t10014: F, t112960: F, t112962: F, t112975: F, t112989: F, t116558: F, t116561: F, t116564: F, t117674: F, t2807: F, t33290: F, t33291: F, t34419: F) -> (F, F) {
    let t118049 = t9740 * t17182 * t34423;
    let t118051 = t17182 * t34428;
    let t118053 = 0.34722222222222222222e-2 * t9740 * t118051;
    let t118064 = t10005 * t33276;
    let t118068 = 0.69841875000000000001e-2 * t34419 * t117674 - 0.69444444444444444445e-2 * t118049 - t118053 + 0.34722222222222222222e-2 * t112960 + 0.34722222222222222222e-2 * t112962 - 0.23214722222222222222e-2 * t116558 + 0.23214722222222222222e-2 * t116561 + 0.77382407407407407407e-3 * t116564 - 0.34722222222222222222e-2 * t112975 - 0.34722222222222222222e-2 * t112989 + 0.13888888888888888889e-1 * t33290 * t10004 * t2807 - 0.30864197530864197531e-2 * t118064 + 0.52083333333333333333e-2 * t33291 * t10014;
    (t118051, t118068)
}
