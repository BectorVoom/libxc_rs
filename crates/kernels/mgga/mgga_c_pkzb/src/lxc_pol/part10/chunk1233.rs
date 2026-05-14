//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1233/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1233<F: Float>(t12: F, t19702: F, t19710: F, t1625: F, t3380: F, t83: F, t16532: F, t1429: F, t439: F, t8: F, t1064: F, t1541: F, t1643: F, t1646: F, t19843: F, t2732: F, t3510: F, t3512: F, t4803: F, t652: F, t78: F, t9150: F, t9155: F, t9158: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t23940 = 0.2077903092681775651e3 * t19702;
    let t23941 = 0.70178683471615754484e1 * t19710;
    let t23943 = t83 * t3380 * t1625;
    let t23944 = 0.32530743900905219526e-1 * t16532;
    let t23948 = t439 * t8 * t1429;
    let t23967 = piecewise3(t84, 0.0, -56.0 / 81.0 * t9150 * t1643 + 64.0 / 27.0 * t2732 * t23948 + 8.0 / 27.0 * t3510 * t1646 - 16.0 / 9.0 * t652 * t78 * t1541 - 8.0 / 9.0 * t1064 * t1429 + 8.0 / 3.0 * t1064 * t4803 + 8.0 / 27.0 * t9155 * t1643 - 4.0 / 9.0 * t9158 * t439 - 2.0 / 9.0 * t3512 * t1646 + t19843);
    (t23940, t23941, t23943, t23944, t23948, t23967)
}
