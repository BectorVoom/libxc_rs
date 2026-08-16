//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 584/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk584(t5351: f64, t863: f64, t1614: f64, t852: f64, t3896: f64, t557: f64, t545: f64, t851: f64, t323: f64, t868: f64, t1308: f64, t880: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5352 = t863 * t5351;
    let t5354 = t852 * t1614;
    let t5359 = 0.13170898365871023197e1_f64 * t3896 * t557;
    let t5360 = t851 * t545;
    let t5361 = t5360 * t323;
    let t5364 = 0.13170898365871023197e1_f64 * t868 * t1614;
    let t5365 = t1308 * t880;
    (t5352, t5354, t5359, t5361, t5364, t5365)
}
