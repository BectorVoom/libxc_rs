//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 492/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk492(t200: f64, t3750: f64, t680: f64, t2379: f64, t3733: f64, t202: f64, t222: f64) -> (f64, f64, f64, f64) {
    let t3751 = t3750 * t200;
    let t3752 = t680 * t3751;
    let t3755 = t2379 * t3733;
    let t3758 = t202 * t222;
    (t3751, t3752, t3755, t3758)
}
