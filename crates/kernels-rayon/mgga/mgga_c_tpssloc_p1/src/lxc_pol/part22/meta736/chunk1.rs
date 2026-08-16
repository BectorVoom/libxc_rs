//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2417/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2417(t21101: f64, t2940: f64, t1581: f64, t49541: f64, t68888: f64, t41684: f64, t48688: f64, t48689: f64, t48698: f64, t59657: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68479: f64, t68483: f64, t68486: f64, t68489: f64, t68492: f64, t68494: f64, t68498: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64) -> (f64, f64, f64) {
    let t68951 = 0.10254018858216406658e4_f64 * t2940 * t21101;
    let t68954 = 0.10526802520742363173e2_f64 * t49541 * t1581 * t68888;
    let t68972 = 0.35616666666666666667e-1_f64 * t68442 + 0.5936111111111111111e-2_f64 * t68444 + 0.65956790123456790123e-2_f64 * t68446 - 0.23744444444444444444e-1_f64 * t68448 + t48688 - t48689 - t48698 + 0.18467901234567901234e-1_f64 * t41684 - 0.52765432098765432099e-1_f64 * t68479 - 0.42739999999999999999e0_f64 * t68483 + 0.2137e0_f64 * t68486 - 0.35616666666666666666e-1_f64 * t68489 - 0.35616666666666666666e-1_f64 * t68492 + 0.11872222222222222222e-1_f64 * t68494 - 0.35616666666666666667e-1_f64 * t68498 - 0.15829629629629629629e-1_f64 * t59657 - 0.17808333333333333333e-1_f64 * t68571 + 0.4274e0_f64 * t68577 - 0.32055e0_f64 * t68580 + 0.10685e0_f64 * t68583;
    (t68951, t68954, t68972)
}
