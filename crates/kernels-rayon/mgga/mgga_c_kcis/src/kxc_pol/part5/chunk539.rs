//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 539/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk539(t895: f64, t898: f64, t227: f64, t897: f64, t224: f64, t906: f64, t2586: f64, t2589: f64, t2591: f64, t2595: f64, t2598: f64, t2601: f64, t2603: f64, t2606: f64, t2608: f64, t2610: f64, t2613: f64, t2616: f64, t2619: f64, t2624: f64) -> (f64, f64, f64, f64, f64) {
    let t2766 = t895 * t898;
    let t2770 = 1.0_f64 / t897 / t227;
    let t2771 = t224 * t2770;
    let t2772 = t906 * t906;
    let t2789 = 0.1875e0_f64 * t2586 - 0.375e0_f64 * t2589 - 0.75e0_f64 * t2591 + 0.375e0_f64 * t2595 + 0.75e0_f64 * t2598 - 0.1875e0_f64 * t2601 + 0.1125e1_f64 * t2603 - 0.4046875e-1_f64 * t2606 + 0.809375e-1_f64 * t2608 + 0.32375e0_f64 * t2610 - 0.809375e-1_f64 * t2613 - 0.32375e0_f64 * t2616 + 0.4046875e-1_f64 * t2619 - 0.809375e0_f64 * t2624;
    (t2766, t2770, t2771, t2772, t2789)
}
