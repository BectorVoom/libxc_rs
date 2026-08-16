//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 967/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk967(t34265: f64, t6210: f64, t1466: f64, t34325: f64, t681: f64, t34321: f64, t1506: f64, t668: f64, t33983: f64, t683: f64, t317: f64, t33953: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t142918 = t6210 * t34265;
    let t142925 = t1466 * t681 * t34325;
    let t142935 = t1466 * t681 * t34321;
    let t142941 = t1506 * t668;
    let t142946 = t683 * t33983;
    let t142950 = t33953 * t317;
    (t142918, t142925, t142935, t142941, t142946, t142950)
}
