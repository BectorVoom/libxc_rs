//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 828/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk828<F: Float>(t25653: F, t5540: F, t25802: F, t22743: F, t25774: F, t25779: F, t401: F, t938: F, t72: F, t1300: F, t1701: F, t22552: F, t22565: F, t22597: F, t22603: F, t22767: F, t22777: F, t22819: F, t22850: F, t22858: F, t25649: F, t25794: F, t25799: F, t25803: F, t25813: F, t25816: F, t25820: F, t5538: F, t5569: F, t5579: F, t5598: F, t5611: F, t6427: F, t6445: F) -> (F, F, F) {
    let t25826 = t5540 * t25653;
    let t25829 = t5540 * t25802;
    let t25832 = t22743 * t25774;
    let t25835 = t5540 * t25779;
    let t25838 = t938 * t401;
    let t25839 = t72 * t25838;
    let t25843 = 0.90822088511484663582e-3 * t22819 * t25794 - 0.12768721675925925926e-1 * t5611 * t25799 - 0.44540303667943584666e-4 * t5569 * t25803 - 0.13784064983740990797e-3 * t5538 * t22777 * t6427 - 0.10214977340740740741e0 * t5598 * t22767 * t6445 + 0.12768721675925925926e-1 * t25813 + 0.17024962234567901235e-1 * t22850 + t22858 - 0.11854761295685025975e-1 * t1300 * t1701 * t25816 - 0.13784064983740990796e-3 * t22565 * t25820 + 0.51690243689028715488e-4 * t22597 * t5540 * t25649 - 0.25845121844514357744e-4 * t22603 * t25826 - 0.51690243689028715488e-5 * t5538 * t25829 - 0.1721820212247325051e-5 * t5538 * t25832 - 0.25845121844514357744e-4 * t22603 * t25835 + 0.76612330055555555556e-1 * t22552 * t5579 * t25839;
    (t25838, t25839, t25843)
}
