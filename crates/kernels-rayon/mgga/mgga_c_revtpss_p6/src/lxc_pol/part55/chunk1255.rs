//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1255/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1255(t125782: f64, t122346: f64, t122435: f64, t122438: f64, t125780: f64, t125785: f64, t125793: f64, t125797: f64, t125799: f64, t125803: f64, t14224: f64, t27846: f64, t32690: f64, t32719: f64) -> f64 {
    let t128770 = 0.263521689745817692e-2_f64 * t125782;
    let t128781 = 0.225875734067843736e-2_f64 * t125780 - t122435 + t128770 + 0.8673628188205199462e0_f64 * t32690 * t27846 + 0.7437465841810202164e-3_f64 * t125785 - t122438 - 0.56468933516960933999e-3_f64 * t125793 + 0.37645955677973955999e-4_f64 * t125797 - 0.66934509195437693771e-4_f64 * t125799 - 0.11423947533020470523e1_f64 * t32719 * t122346 * t14224 + 0.112937867033921868e-1_f64 * t125803;
    t128781
}
