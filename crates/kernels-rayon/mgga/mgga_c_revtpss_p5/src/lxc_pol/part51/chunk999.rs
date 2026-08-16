//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 999/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk999(t33800: f64, t8521: f64, t1695: f64, t373: f64, t372: f64, t371: f64, t1674: f64, t31993: f64, t1665: f64, t1671: f64, t31885: f64, t31891: f64, t31903: f64, t31934: f64, t31950: f64, t31992: f64, t32000: f64, t32003: f64, t32010: f64, t33797: f64, t33804: f64, t33808: f64, t33812: f64, t33817: f64, t8517: f64, t8524: f64) -> (f64, f64, f64, f64, f64) {
    let t33822 = t33800 * t8521;
    let t33825 = t373 * t1695;
    let t33826 = t372 * t33825;
    let t33827 = t371 * t33826;
    let t33832 = t31993 * t1674;
    let t33835 = -0.17135921299530705785e1_f64 * t31903 * t33797 + 0.57119737665102352616e0_f64 * t33800 * t8517 - 0.17135921299530705785e1_f64 * t31891 * t33804 + 0.11423947533020470523e1_f64 * t31934 * t33808 + 0.11423947533020470523e1_f64 * t31891 * t33812 - 0.5578099381357651623e-3_f64 * t32003 * t33817 + 0.5578099381357651623e-3_f64 * t32010 * t1665 - 0.1859366460452550541e-3_f64 * t33822 * t8524 + 0.3718732920905101082e-3_f64 * t31950 * t33827 - 0.3718732920905101082e-3_f64 * t32000 * t1671 - t31885 - 0.12395776403017003607e-3_f64 * t31992 * t33832;
    (t33822, t33825, t33827, t33832, t33835)
}
