//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 960/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk960(t11695: f64, t3225: f64, t773: f64, t826: f64, t10264: f64, t3212: f64, t3724: f64, t3209: f64, t11659: f64, t11664: f64, t11666: f64, t11671: f64, t11676: f64, t11680: f64, t11685: f64, t11689: f64, t11692: f64) -> (f64, f64) {
    let t11696 = t3225 * t11695;
    let t11698 = t826 * t773;
    let t11699 = t10264 * t11698;
    let t11701 = t3212 * t3724;
    let t11703 = t3209 * t3724;
    let t11705 = -0.24464544158376474785e-7_f64 * t11659 + 0.32109714207869123156e-6_f64 * t11664 - 0.11742981196020707897e-4_f64 * t11666 - 0.11742981196020707897e-4_f64 * t11671 + 0.17098714139140853038e-6_f64 * t11676 + 0.17098714139140853038e-6_f64 * t11680 - 0.73393632475129424356e-6_f64 * t11685 - 0.73393632475129424356e-6_f64 * t11689 - 0.20879020566524818641e-5_f64 * t11692 + 0.56995713797136176793e-7_f64 * t11696 - 0.27357942622625364861e-5_f64 * t11699 + 0.82073827867876094584e-5_f64 * t11701 - 0.3556532540941297432e-4_f64 * t11703;
    (t11698, t11705)
}
