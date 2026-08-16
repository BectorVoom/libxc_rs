//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 999/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk999<F: Float>(t33800: F, t8521: F, t1695: F, t373: F, t372: F, t371: F, t1674: F, t31993: F, t1665: F, t1671: F, t31885: F, t31891: F, t31903: F, t31934: F, t31950: F, t31992: F, t32000: F, t32003: F, t32010: F, t33797: F, t33804: F, t33808: F, t33812: F, t33817: F, t8517: F, t8524: F) -> (F, F, F, F, F) {
    let t33822 = t33800 * t8521;
    let t33825 = t373 * t1695;
    let t33826 = t372 * t33825;
    let t33827 = t371 * t33826;
    let t33832 = t31993 * t1674;
    let t33835 = -F::cast_from(0.17135921299530705785e1_f64) * t31903 * t33797 + F::cast_from(0.57119737665102352616e0_f64) * t33800 * t8517 - F::cast_from(0.17135921299530705785e1_f64) * t31891 * t33804 + F::cast_from(0.11423947533020470523e1_f64) * t31934 * t33808 + F::cast_from(0.11423947533020470523e1_f64) * t31891 * t33812 - F::cast_from(0.5578099381357651623e-3_f64) * t32003 * t33817 + F::cast_from(0.5578099381357651623e-3_f64) * t32010 * t1665 - F::cast_from(0.1859366460452550541e-3_f64) * t33822 * t8524 + F::cast_from(0.3718732920905101082e-3_f64) * t31950 * t33827 - F::cast_from(0.3718732920905101082e-3_f64) * t32000 * t1671 - t31885 - F::cast_from(0.12395776403017003607e-3_f64) * t31992 * t33832;
    (t33822, t33825, t33827, t33832, t33835)
}
