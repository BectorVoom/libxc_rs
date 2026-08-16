//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 653/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk653(t1909: f64, t310: f64, t1220: f64, t1658: f64, t556: f64, t1907: f64, t315: f64, t323: f64, t1914: f64, t3875: f64, t463: f64, t3880: f64, t3890: f64, t3893: f64, t3900: f64, t3902: f64, t4103: f64, t4130: f64, t4133: f64, t4139: f64, t446: f64, t557: f64) -> (f64, f64, f64) {
    let t6422 = t310 * t1909;
    let t6425 = t1220 * t556 * t1658;
    let t6434 = t315 * t1907;
    let t6435 = t6434 * t323;
    let t6438 = t3875 * t1914 * t463;
    let t6441 = 0.65854491829355115987e0_f64 * t6422 - t4130 - t4133 + 0.26341796731742046394e1_f64 * t446 * t6425 + 0.65854491829355115987e0_f64 * t3880 - t4139 + 0.65854491829355115987e0_f64 * t3890 - 0.13170898365871023197e1_f64 * t4103 * t557 - 0.65854491829355115987e0_f64 * t3893 - t3900 - 0.13170898365871023197e1_f64 * t3902 - 0.65854491829355115987e0_f64 * t6435 - 0.39512695097613069591e1_f64 * t446 * t6438;
    (t6425, t6438, t6441)
}
