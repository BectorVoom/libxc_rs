//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1103/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1103(t1013: f64, t7189: f64, t137007: f64, t538: f64, t26701: f64, t32152: f64, t32174: f64, t34877: f64, t173: f64, t34876: f64, t7195: f64, t23825: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t147256 = t7189 * t1013;
    let t147258 = t137007 * t147256 * t538;
    let t147262 = t32152 * t26701;
    let t147266 = t32174 * t34877;
    let t147270 = t7195 * t173 * t34876;
    let t147271 = t23825 * t147270;
    (t147256, t147258, t147262, t147266, t147270, t147271)
}
