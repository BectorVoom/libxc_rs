//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1193/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1193(t3014: f64, t972: f64, t4732: f64, t981: f64, t2848: f64, t3037: f64, t4571: f64, t4576: f64, t4581: f64, t4585: f64, t341: f64) -> (f64, f64, f64, f64, f64) {
    let t4733 = t3014 * t972;
    let t4734 = t4732 * t4733;
    let t4736 = 0.17315859105681463759e2_f64 * t981 * t4734;
    let t4742 = t3037 + 0.27777777777777777778e-2_f64 * t2848 + 0.27777777777777777778e-2_f64 * t4571 - 0.55555555555555555555e-2_f64 * t4576 + 0.16666666666666666667e-1_f64 * t4581 - 0.83333333333333333333e-2_f64 * t4585;
    let t4743 = t4742 * t341;
    (t4733, t4734, t4736, t4742, t4743)
}
