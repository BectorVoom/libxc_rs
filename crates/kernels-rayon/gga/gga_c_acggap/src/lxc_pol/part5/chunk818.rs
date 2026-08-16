//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 818/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk818(t182: f64, t6413: f64, t1251: f64, t1839: f64, t1925: f64, t377: f64, t1411: f64, t1651: f64, t119: f64, t151: f64, t3827: f64, t4235: f64, t4244: f64, t4246: f64, t6495: f64, t6498: f64, t6501: f64, t6503: f64, t6507: f64, t6510: f64, t6513: f64) -> (f64, f64, f64, f64) {
    let t6515 = t182 * t6413;
    let t6518 = t1251 * t1839;
    let t6521 = t377 * t1925;
    let t6523 = t1651 * t1411;
    let t6526 = -0.65854491829355115987e0_f64 * t151 * t6495 - 0.65854491829355115987e0_f64 * t6498 + 0.13170898365871023197e1_f64 * t4235 - 0.65854491829355115987e0_f64 * t6501 - 0.65854491829355115987e0_f64 * t151 * t6503 - t4244 + 0.26341796731742046394e1_f64 * t4246 - t3827 - 0.13170898365871023197e1_f64 * t151 * t6507 + 0.13170898365871023197e1_f64 * t151 * t6510 + 0.65854491829355115987e0_f64 * t6513 + 0.65854491829355115987e0_f64 * t119 * t6515 - 0.65854491829355115987e0_f64 * t151 * t6518 - 0.13170898365871023197e1_f64 * t6521 - 0.13170898365871023197e1_f64 * t151 * t6523;
    (t6515, t6518, t6523, t6526)
}
