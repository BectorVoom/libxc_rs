//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 837/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk837(t5: f64, t1675: f64, t1792: f64, t5483: f64, t5489: f64, t5492: f64, t5785: f64, t5793: f64, t5794: f64, t117: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t5798 = piecewise3(t8, 0.0_f64, t5483 * t1792 / 3.0_f64 - 5.0_f64 / 3.0_f64 * t5785 * t5489 - 2.0_f64 / 3.0_f64 * t5492 * t1792 - t5793 + t1675 * t5794 / 3.0_f64);
    let t5799 = t5798 * t117;
    (t5798, t5799)
}
