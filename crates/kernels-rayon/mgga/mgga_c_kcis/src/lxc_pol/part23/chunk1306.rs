//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1306/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1306(t27651: f64, t8209: f64, t27556: f64, t28772: f64, t94621: f64, t94624: f64, t95130: f64, t98663: f64, t98666: f64, t98673: f64, t98676: f64, t98680: f64, t98684: f64) -> f64 {
    let t99524 = t8209 * t27651;
    let t99534 = 0.7722800925925925926e-4_f64 * t95130 + 0.46429444444444444443e-2_f64 * t98663 - 0.15476481481481481481e-2_f64 * t98666 + 0.7722800925925925926e-4_f64 * t99524 - 0.17411041666666666666e-2_f64 * t98673 + 0.61905925925925925924e-2_f64 * t98676 + 0.23214722222222222222e-2_f64 * t98680 + 0.51588271604938271604e-3_f64 * t98684 + 0.92754700520833333334e-4_f64 * t27556 * t28772 - 0.25794135802469135802e-3_f64 * t94621 - 0.23214722222222222222e-2_f64 * t94624;
    t99534
}
