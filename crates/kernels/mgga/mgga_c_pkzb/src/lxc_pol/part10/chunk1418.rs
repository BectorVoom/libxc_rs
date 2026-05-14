//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1418/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1418<F: Float>(t1062: F, t1162: F, t1469: F, t1821: F, t1824: F, t195: F, t199: F, t201: F, t23683: F, t23891: F, t23915: F, t23922: F, t23939: F, t2476: F, t24931: F, t24951: F, t24952: F, t24957: F, t24960: F, t24986: F, t25003: F, t25019: F, t25026: F, t25029: F, t25045: F, t25051: F, t25070: F, t2531: F, t26840: F, t2724: F, t28624: F, t328: F, t330: F, t3359: F, t3507: F, t3715: F, t3719: F, t45: F, t462: F, t642: F, t645: F, t6750: F, t7217: F, t8592: F, t8708: F, t9127: F, t9743: F, t998: F) -> (F,) {
    let t28628 = (t23683 + t23891) * t195 + 2.0 * t8708 * t642 + t3359 * t1821 + 2.0 * t6750 * t1062 + 4.0 * t2531 * t2724 + 2.0 * t998 * t7217 + t1469 * t3507 + 2.0 * t462 * t9127 + t45 * (t23915 + t23922 + t23939 + t24931 + t24951 + t24952 + t24957 + t24960 + t24986 + t25003 + t25019 + t25026 + t25029 + t25045 + t25051 + t25070) - 0.1434375e0 * t1824 * t3715 + 0.95625e-1 * t645 * t9743 - 0.2390625e-1 * t199 * t201 * t26840 - 0.1434375e0 * t3719 * t2476 + 0.95625e-1 * t1162 * t8592 - 0.2390625e-1 * t328 * t330 * t28624;
    (t28628,)
}
