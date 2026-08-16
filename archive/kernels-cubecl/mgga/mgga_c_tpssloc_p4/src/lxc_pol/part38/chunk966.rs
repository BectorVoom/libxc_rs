//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 966/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk966<F: Float>(t10770: F, t302: F, t10294: F, t10544: F, t2884: F, t922: F, t2887: F, t310: F, t2791: F, t888: F, t2897: F, t942: F) -> (F, F, F, F, F, F, F) {
    let t10771 = t302 * t10770;
    let t10784 = F::cast_from(0.46308888888888888888e0_f64) * t10294;
    let t10785 = F::cast_from(0.16068111111111111111e1_f64) * t10544;
    let t10810 = F::cast_from(1.0_f64) / t2884 / t922;
    let t10811 = t302 * t10810;
    let t10813 = F::cast_from(1.0_f64) / t2887 / t310;
    let t10817 = t888 * t2791;
    let t10820 = t2897 * t942;
    (t10771, t10784, t10785, t10811, t10813, t10817, t10820)
}
