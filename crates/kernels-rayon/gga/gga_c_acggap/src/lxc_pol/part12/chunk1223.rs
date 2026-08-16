//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1223/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1223(t7990: f64, t9436: f64, t2176: f64, t5379: f64, t1614: f64, t8111: f64, t2131: f64, t2132: f64, t309: f64, t9367: f64, t33175: f64, t7963: f64, t9029: f64) -> (f64, f64, f64, f64, f64) {
    let t38015 = 0.17347256376410398924e1_f64 * t7990 * t9436;
    let t38018 = 0.13170898365871023197e1_f64 * t2176 * t5379;
    let t38019 = t8111 * t1614;
    let t38033 = 0.17347256376410398924e1_f64 * t2131 * t2132 * t9367 * t309;
    let t38036 = 0.17347256376410398924e1_f64 * t7963 * t33175 * t9029;
    (t38015, t38018, t38019, t38033, t38036)
}
