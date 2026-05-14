//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 909/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk909<F: Float>(t136433: F, t3076: F, t1554: F, t1570: F, t3188: F, t32241: F, t1557: F, t7760: F, t358: F, t137037: F, t3033: F, t136815: F, t1630: F, t3037: F, t136282: F, t136365: F, t136369: F, t136434: F, t136604: F, t136814: F, t1669: F, t22597: F, t25698: F, t25703: F, t25788: F, t25826: F, t25835: F, t9: F, t92809: F) -> (F, F, F, F, F, F) {
    let t145160 = t3076 * t136433;
    let t145163 = t32241 * t1554 * t1570 * t3188;
    let t145168 = t32241 * t7760 * t1557 * t3188;
    let t145171 = t1554 * t358;
    let t145188 = t137037 * t3033;
    let t145192 = t136815 * t1630 * t3037;
    let t145195 = -0.10338048737805743097e-3 * t136604 * t25826 - 0.78259321553885081522e-2 * t145160 * t145163 + 0.65216101294904234602e-2 * t145160 * t145168 + 0.78259321553885081522e-2 * t136434 * t32241 * t145171 * t25698 - 0.11738898233082762228e-1 * t136282 * t32241 * t145171 * t25703 - 0.10338048737805743097e-3 * t136604 * t25835 - 0.45967398033333333333e0 * t1669 * t92809 * t9 * t25788 - 0.13200366700519885118e-5 * t136365 + 0.29693535778629056444e-3 * t136369 + 0.25845121844514357744e-4 * t136814 * t145188 + 0.51690243689028715488e-5 * t22597 * t145192;
    (t145163, t145168, t145171, t145188, t145192, t145195)
}
