//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1071/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1071(t17748: f64, t4531: f64, t4540: f64, t7577: f64, t4546: f64, t343: f64, t5842: f64, t984: f64, t2970: f64, t5824: f64, t973: f64, t10226: f64, t13782: f64, t13787: f64, t13790: f64, t13825: f64, t17742: f64, t17745: f64, t2960: f64, t2986: f64, t5825: f64) -> f64 {
    let t17749 = t4531 * t17748;
    let t17752 = t7577 * t4540;
    let t17753 = t4546 * t17752;
    let t17757 = t5842 * t984 * t343;
    let t17758 = t4546 * t17757;
    let t17763 = t2970 * t5824;
    let t17764 = t973 * t17763;
    let t17766 = -t13782 + t13787 - t13790 - 0.6172839506172839506e-4_f64 * t10226 + 0.11111111111111111111e-2_f64 * t2986 * t17742 - 0.74074074074074074072e-3_f64 * t2986 * t17745 - 0.55555555555555555554e-3_f64 * t2986 * t17749 - 0.16666666666666666666e-2_f64 * t973 * t17753 - 0.83333333333333333332e-3_f64 * t973 * t17758 + 0.14814814814814814814e-2_f64 * t2960 * t5825 - 0.18518518518518518518e-3_f64 * t17764 - t13825;
    t17766
}
