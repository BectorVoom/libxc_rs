//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2429/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2429<F: Float>(t273: F, t41654: F, t242: F, t281: F, t283: F, t2853: F, t2860: F, t10770: F, t919: F, t2897: F, t2904: F, t10701: F, t888: F) -> (F, F, F, F, F, F, F, F) {
    let t41942 = F::powf(t273, -F::cast_from(0.25e1_f64));
    let t41959 = F::cast_from(0.31310740740740740741e1_f64) * t41654;
    let t41961 = t281 * t242 * t283;
    let t41962 = F::cast_from(0.13490888888888888889e1_f64) * t41961;
    let t41981 = t2853 * t2860;
    let t41984 = t919 * t10770;
    let t42020 = t2897 * t2904;
    let t42023 = t888 * t10701;
    (t41942, t41959, t41961, t41962, t41981, t41984, t42020, t42023)
}
