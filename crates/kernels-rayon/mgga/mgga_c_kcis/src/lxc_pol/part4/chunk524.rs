//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 524/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk524(t165: f64, t2531: f64, t779: f64, t782: f64, t826: f64, t164: f64, t781: f64, t142: f64, t143: f64, t2379: f64, t126: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2532 = t2531 * t165;
    let t2533 = t779 * t782;
    let t2534 = t2533 * t826;
    let t2535 = 2.0_f64 * t2534;
    let t2537 = 1.0_f64 / t781 / t164;
    let t2538 = t142 * t2537;
    let t2539 = t826 * t826;
    let t2540 = t2538 * t2539;
    let t2541 = 2.0_f64 * t2540;
    let t2542 = t2379 * t143;
    let t2545 = t684 * t126;
    (t2532, t2533, t2534, t2535, t2537, t2538, t2539, t2540, t2541, t2542, t2545)
}
