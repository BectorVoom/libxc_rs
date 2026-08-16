//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1780/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1780<F: Float>(t300: F, t90745: F, t90775: F, t90805: F, t90852: F, t24488: F, t5192: F, t1196: F, t20890: F, t69511: F, t6535: F, t6555: F) -> (F, F, F, F) {
    let t90855 = t300 * (t90745 + t90775 + t90805 + t90852);
    let t90857 = F::cast_from(0.14035736694323150897e2_f64) * t5192 * t24488;
    let t90860 = F::cast_from(0.61524113149298439947e4_f64) * t1196 * t20890 * t69511;
    let t90863 = F::cast_from(0.21053605041484726346e2_f64) * t1196 * t6555 * t6535;
    (t90855, t90857, t90860, t90863)
}
