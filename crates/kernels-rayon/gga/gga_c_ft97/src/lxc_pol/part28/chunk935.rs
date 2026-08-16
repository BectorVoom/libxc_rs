//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 935/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk935(t136240: f64, t32071: f64, t23054: f64, t32096: f64, t32356: f64, t376: f64, t89: f64, t1557: f64, t7165: f64, t17: f64, t171: f64, t397: f64) -> (f64, f64, f64, f64, f64) {
    let t136241 = t136240 * t32071;
    let t136243 = t23054 * t32096;
    let t136250 = t89 * t376 * t32356;
    let t136269 = t7165 * t1557;
    let t136275 = t397 * t171 * t17;
    (t136241, t136243, t136250, t136269, t136275)
}
