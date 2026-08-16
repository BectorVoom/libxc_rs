//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 404/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk404(t119: f64, t1306: f64, t1309: f64, t1608: f64, t1611: f64, t1615: f64, t1909: f64, t1915: f64, t1938: f64, t446: f64, t557: f64, t850: f64, t854: f64, t867: f64, t882: f64) -> f64 {
    let t1941 = t850 - t854 + 0.13170898365871023197e1_f64 * t1306 - 0.13170898365871023197e1_f64 * t1611 + t867 - 0.13170898365871023197e1_f64 * t1309 + 0.13170898365871023197e1_f64 * t1615 - t882 + 0.65854491829355115987e0_f64 * t119 * t1909 - 0.13170898365871023197e1_f64 * t1608 * t557 + 0.13170898365871023197e1_f64 * t446 * t1915 - 0.65854491829355115987e0_f64 * t446 * t1938;
    t1941
}
