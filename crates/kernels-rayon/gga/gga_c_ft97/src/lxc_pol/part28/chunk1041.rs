//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1041/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1041(t136282: f64, t136365: f64, t136369: f64, t136434: f64, t136604: f64, t136814: f64, t145160: f64, t145163: f64, t145168: f64, t145171: f64, t145188: f64, t145192: f64, t1669: f64, t22597: f64, t25698: f64, t25703: f64, t25788: f64, t25826: f64, t25835: f64, t32241: f64, t9: f64, t92809: f64) -> f64 {
    let t145195 = -0.10338048737805743097e-3_f64 * t136604 * t25826 - 0.78259321553885081522e-2_f64 * t145160 * t145163 + 0.65216101294904234602e-2_f64 * t145160 * t145168 + 0.78259321553885081522e-2_f64 * t136434 * t32241 * t145171 * t25698 - 0.11738898233082762228e-1_f64 * t136282 * t32241 * t145171 * t25703 - 0.10338048737805743097e-3_f64 * t136604 * t25835 - 0.45967398033333333333e0_f64 * t1669 * t92809 * t9 * t25788 - 0.13200366700519885118e-5_f64 * t136365 + 0.29693535778629056444e-3_f64 * t136369 + 0.25845121844514357744e-4_f64 * t136814 * t145188 + 0.51690243689028715488e-5_f64 * t22597 * t145192;
    t145195
}
