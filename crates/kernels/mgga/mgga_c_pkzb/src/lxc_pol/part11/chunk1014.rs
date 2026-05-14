//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1014/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1014<F: Float>(t735: F, t7620: F, t17867: F, t2104: F, t2911: F, t2064: F, t2922: F, t2924: F, t2918: F, t5939: F, t757: F, t771: F, t7755: F, t1066: F, t179: F, t18107: F, t299: F) -> (F, F, F, F, F, F) {
    let t21542 = t735 * t7620;
    let t21543 = t21542 / 54.0;
    let t21623 = t2104 * t17867 * t2911;
    let t21624 = 0.28582678745379824648e-3 * t21623;
    let t21626 = t2922 * t2064 * t2924;
    let t21627 = 0.14291339372689912324e-3 * t21626;
    let t21651 = t757 * t5939 * t2918;
    let t21652 = 0.14291339372689912324e-3 * t21651;
    let t21657 = t771 * t7755;
    let t21658 = 0.15244095330869239812e-2 * t21657;
    let t21661 = t299 * t179 * t18107 * t1066;
    (t21543, t21624, t21627, t21652, t21658, t21661)
}
