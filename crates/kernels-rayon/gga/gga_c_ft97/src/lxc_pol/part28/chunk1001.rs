//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1001/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1001(t34614: f64, t5498: f64, t31998: f64, t6414: f64, t1286: f64, t1337: f64, t136059: f64, t25528: f64, t25533: f64, t25584: f64, t25602: f64, t28: f64, t3103: f64, t32002: f64, t32016: f64, t32054: f64, t32338: f64, t32385: f64, t32403: f64, t5507: f64, t5510: f64, t6461: f64, t7168: f64, t7218: f64) -> f64 {
    let t144416 = t34614 * t5498;
    let t144420 = t6414 * t31998;
    let t144442 = -t6414 * t32002 / 3.0_f64 + t32016 * t25602 / 9.0_f64 - t144416 / 18.0_f64 + t25584 * t7218 / 3.0_f64 - t144420 / 18.0_f64 + t32054 * t6461 / 6.0_f64 - t34614 * t5510 / 3.0_f64 - t25584 * t7168 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t5507 * t1337 * t3103 + t1286 * t28 * t32338 * t25533 + t6414 * t32403 - 2.0_f64 / 3.0_f64 * t1286 * t28 * t25528 * t32385 - t136059 / 18.0_f64;
    t144442
}
