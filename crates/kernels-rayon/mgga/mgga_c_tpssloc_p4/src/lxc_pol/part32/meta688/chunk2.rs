//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2132/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2132(t28030: f64, t6535: f64, t26114: f64, t7461: f64, t19994: f64, t24995: f64, t8945: f64, t28831: f64, t83886: f64, t6287: f64, t652: f64, t6534: f64) -> (f64, f64, f64, f64, f64) {
    let t96738 = 2.0_f64 * t28030 * t6535;
    let t96740 = 4.0_f64 * t26114 * t7461;
    let t96746 = 6.0_f64 * t24995 * t8945 * t19994;
    let t96755 = 6.0_f64 * t83886 * t28831;
    let t96758 = 2.0_f64 * t652 * t6287 * t6534;
    (t96738, t96740, t96746, t96755, t96758)
}
