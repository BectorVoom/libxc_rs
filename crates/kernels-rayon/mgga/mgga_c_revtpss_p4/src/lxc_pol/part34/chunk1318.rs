//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1318/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1318(t30138: f64, t7742: f64, t30128: f64, t4248: f64, t1937: f64, t75941: f64, t114373: f64, t18245: f64, t7735: f64, t22852: f64, t28167: f64, t8996: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t114434 = 12.0_f64 * t30138 * t7742;
    let t114436 = 6.0_f64 * t4248 * t30128;
    let t114438 = 2.0_f64 * t75941 * t1937;
    let t114440 = 6.0_f64 * t114373 * t1937;
    let t114442 = 6.0_f64 * t18245 * t7735;
    let t114445 = 18.0_f64 * t28167 * t8996 * t22852;
    (t114434, t114436, t114438, t114440, t114442, t114445)
}
