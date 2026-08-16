//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 715/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk715(t11281: f64, t419: f64, t1725: f64, t3092: f64, t11255: f64, t11260: f64, t11265: f64, t11267: f64, t11271: f64, t11275: f64, t11278: f64, t8074: f64, t8079: f64, t8086: f64) -> (f64, f64, f64) {
    let t11282 = t419 * t11281;
    let t11284 = t1725 * t3092;
    let t11286 = 0.12768721675925925926e-1_f64 * t11255 + 0.2269994964609053498e-1_f64 * t8074 + t8079 + 0.62424861526748971195e-1_f64 * t8086 - 0.85124811172839506173e-2_f64 * t11260 + t11265 + 0.85124811172839506173e-2_f64 * t11267 + 0.19862455940329218107e-1_f64 * t11271 + 0.3404992446913580247e-1_f64 * t11275 - 0.12768721675925925926e-1_f64 * t11278 - 0.51074886703703703704e-1_f64 * t11282 + 0.68099848938271604939e-1_f64 * t11284;
    (t11282, t11284, t11286)
}
