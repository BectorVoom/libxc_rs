//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 553/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk553<F: Float>(t1124: F, t3799: F, t2441: F, t4917: F, t420: F, t701: F, t2446: F, t4635: F, t704: F, t2435: F, t3796: F, t3804: F, t5031: F, t695: F, t1111: F, t1115: F, t1417: F, t1701: F, t224: F, t238: F, t2384: F, t2387: F, t3759: F, t3766: F, t4940: F, t4943: F, t4949: F, t4953: F, t4957: F, t4961: F, t4979: F, t4982: F, t4987: F, t4991: F, t5003: F, t5007: F, t5016: F, t5019: F, t5026: F, t678: F, t680: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5034 = t3799 * t1124;
    let t5037 = t2441 * t4917;
    let t5038 = t420 * t5037;
    let t5039 = t701 * t5038;
    let t5041 = t2446 * t4917;
    let t5042 = t420 * t5041;
    let t5043 = t701 * t5042;
    let t5045 = t704 * t4635;
    let t5046 = t420 * t5045;
    let t5047 = t701 * t5046;
    let t5049 = 0.18727458458024691358e0 * t5031 - 0.3404992446913580247e-1 * t3796 - 0.3404992446913580247e-1 * t5034 - t2435 + 0.42562405586419753086e-2 * t3804 + 0.85124811172839506173e-2 * t5039 - 0.12768721675925925926e-1 * t5043 + 0.6384360837962962963e-2 * t5047;
    let t5050 = t695 * t5049;
    let t5052 = 0.67598802253579164263e-4 * t4940 * t2384 - 0.46509801892875584e-1 * t3759 * t680 * t4943 - 0.13784064983740990796e-3 * t4949 * t4953 + 0.23254900946437792e-1 * t2387 * t4957 + 0.23254900946437792e-2 * t678 * t4961 - 0.11627450473218896e-1 * t678 * t4979 + 0.19365723406274399941e-3 * t678 * t4982 + 2.0 * t4987 + 0.2370952259137005195e-1 * t1115 * t1111 - 4.0 * t3766 * t4991 + 2.0 * t5007 + 0.14053536537767171586e-3 * t238 * t5016 - 0.11854761295685025975e-1 * t1417 * t1701 * t5019 - 0.37540077436335915588e-1 * t238 * t5003 + 2.0 * t224 * t5026 - t224 * t5050;
    (t5034, t5037, t5039, t5041, t5043, t5045, t5047, t5049, t5052)
}
