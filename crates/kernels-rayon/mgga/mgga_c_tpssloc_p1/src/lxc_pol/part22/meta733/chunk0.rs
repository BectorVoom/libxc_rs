//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2404/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2404(t13515: f64, t5727: f64, t17423: f64, t4354: f64, t49269: f64, t5730: f64, t21268: f64, t42143: f64, t21300: f64, t2787: f64, t47705: f64, t47707: f64, t48103: f64, t48919: f64, t48924: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68767 = 3.0_f64 * t13515 * t5727;
    let t68769 = 3.0_f64 * t4354 * t17423;
    let t68771 = 0.48245938496077605201e2_f64 * t49269 * t5730;
    let t68773 = 0.96491876992155210402e2_f64 * t42143 * t21268;
    let t68775 = 1.0_f64 * t2787 * t21300;
    let t68785 = 0.59793333333333333333e0_f64 * t68442 + 0.99655555555555555557e-1_f64 * t68444 + 0.11072839506172839506e0_f64 * t68446 - 0.39862222222222222223e0_f64 * t68448 + 0.79724444444444444446e0_f64 * t47705 - 0.26574814814814814815e0_f64 * t47707 - t48919 - t48924 - 0.32862666666666666666e0_f64 * t68452 + 0.54771111111111111112e-1_f64 * t68454 + 0.73028148148148148149e0_f64 * t48103;
    (t68767, t68769, t68771, t68773, t68775, t68785)
}
