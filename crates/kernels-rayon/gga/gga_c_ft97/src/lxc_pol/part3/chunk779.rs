//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 779/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk779(t110: f64, t15772: f64, t447: f64, t1866: f64, t4454: f64, t499: f64, t432: f64, t4436: f64, t8411: f64, t492: f64, t1871: f64, t488: f64) -> (f64, f64, f64, f64) {
    let t16103 = t447 * t110 * t15772;
    let t16107 = t1866 * t499 * t4454;
    let t16110 = t4436 * t432;
    let t16112 = t8411 * t110 * t16110;
    let t16115 = t4436 * t492;
    let t16117 = t1871 * t488 * t16115;
    (t16103, t16107, t16112, t16117)
}
