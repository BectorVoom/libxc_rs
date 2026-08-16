//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 548/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk548(t323: f64, t4131: f64, t310: f64, t545: f64, t464: f64, t1410: f64, t180: f64, t1539: f64, t1160: f64, t157: f64, t879: f64, t1629: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4133 = 0.13170898365871023197e1_f64 * t4131 * t323;
    let t4137 = t310 * t545;
    let t4139 = 0.13170898365871023197e1_f64 * t4137 * t464;
    let t4146 = t180 * t1410;
    let t4150 = t4146 * t1539;
    let t4152 = 0.13170898365871023197e1_f64 * t1160 * t4150;
    let t4162 = t157 * t879;
    let t4163 = t1629 * t4162;
    (t4133, t4137, t4139, t4152, t4162, t4163)
}
