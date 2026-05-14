//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 916/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk916<F: Float>(t1123: F, t5728: F, t2027: F, t287: F, t302: F, t2739: F, t759: F, t761: F, t2105: F, t1066: F, t2009: F, t2031: F, t1120: F, t2057: F, t2104: F, t276: F, t2895: F, t2899: F, t2922: F, t2933: F, t5646: F, t5661: F, t5666: F, t5984: F, t735: F, t7621: F, t7630: F, t7632: F, t7639: F, t7642: F, t7650: F, t7655: F, t7660: F, t7664: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7665 = t1123 * t5728;
    let t7666 = t2027 * t287;
    let t7667 = t7665 * t7666;
    let t7668 = t302 * t7667;
    let t7671 = t2739 * t759;
    let t7672 = t7671 * t761;
    let t7673 = t2105 * t7672;
    let t7676 = t1066 * t2009;
    let t7677 = t7676 * t761;
    let t7678 = t2105 * t7677;
    let t7681 = t1066 * t2027;
    let t7682 = t7681 * t2031;
    let t7683 = t2105 * t7682;
    let t7686 = t7621 / 432.0 - 11.0 / 108.0 * t2057 * t1120 + t735 * t2895 / 18.0 - t7630 - t276 * t7632 / 96.0 + 0.45732285992607719436e-2 * t5984 * t2933 - t7639 + 0.12862205435420921092e-2 * t2104 * t7642 - t5646 / 288.0 + t5661 / 54.0 + t5666 / 144.0 - 0.42874018118069736972e-3 * t2104 * t7650 - 0.42874018118069736972e-3 * t2922 * t7655 - 0.21437009059034868486e-3 * t2922 * t7660 + 0.21437009059034868486e-3 * t7664 * t7668 - 0.85748036236139473944e-3 * t2104 * t7673 - 0.42874018118069736972e-3 * t2104 * t7678 - 0.85748036236139473944e-3 * t2899 * t7683;
    (t7665, t7666, t7667, t7668, t7672, t7673, t7677, t7678, t7681, t7682, t7683, t7686)
}
