//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 610/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk610(t25779: f64, t5540: f64, t401: f64, t938: f64, t72: f64, t1300: f64, t1701: f64, t22552: f64, t22565: f64, t22597: f64, t22603: f64, t22767: f64, t22777: f64, t22819: f64, t22850: f64, t22858: f64, t25649: f64, t25794: f64, t25799: f64, t25803: f64, t25813: f64, t25816: f64, t25820: f64, t25826: f64, t25829: f64, t25832: f64, t5538: f64, t5569: f64, t5579: f64, t5598: f64, t5611: f64, t6427: f64, t6445: f64) -> (f64, f64, f64, f64) {
    let t25835 = t5540 * t25779;
    let t25838 = t938 * t401;
    let t25839 = t72 * t25838;
    let t25843 = 0.90822088511484663582e-3_f64 * t22819 * t25794 - 0.12768721675925925926e-1_f64 * t5611 * t25799 - 0.44540303667943584666e-4_f64 * t5569 * t25803 - 0.13784064983740990797e-3_f64 * t5538 * t22777 * t6427 - 0.10214977340740740741e0_f64 * t5598 * t22767 * t6445 + 0.12768721675925925926e-1_f64 * t25813 + 0.17024962234567901235e-1_f64 * t22850 + t22858 - 0.11854761295685025975e-1_f64 * t1300 * t1701 * t25816 - 0.13784064983740990796e-3_f64 * t22565 * t25820 + 0.51690243689028715488e-4_f64 * t22597 * t5540 * t25649 - 0.25845121844514357744e-4_f64 * t22603 * t25826 - 0.51690243689028715488e-5_f64 * t5538 * t25829 - 0.1721820212247325051e-5_f64 * t5538 * t25832 - 0.25845121844514357744e-4_f64 * t22603 * t25835 + 0.76612330055555555556e-1_f64 * t22552 * t5579 * t25839;
    (t25835, t25838, t25839, t25843)
}
