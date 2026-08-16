//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1255/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1255(t11523: f64, t25842: f64, t1743: f64, t19511: f64, t34090: f64, t11329: f64, t9262: f64, t27063: f64, t3709: f64, t26017: f64, t19771: f64, t3718: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34951 = t11523 * t25842;
    let t34954 = t1743 * t34090 * t19511;
    let t34956 = t11329 * t9262;
    let t34958 = t3709 * t27063;
    let t34960 = t3709 * t26017;
    let t34962 = t3718 * t19771;
    (t34951, t34954, t34956, t34958, t34960, t34962)
}
