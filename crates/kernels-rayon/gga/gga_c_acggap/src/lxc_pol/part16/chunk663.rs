//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 663/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk663(t6479: f64, t6493: f64, t6526: f64, t6555: f64, t449: f64, t1937: f64, t463: f64, t1220: f64, t1215: f64, t1608: f64, t1659: f64, t1915: f64, t3932: f64, t3935: f64, t3939: f64, t446: f64, t5365: f64, t5369: f64, t5372: f64, t5381: f64, t5382: f64, t5388: f64) -> (f64, f64, f64, f64) {
    let t6557 = t6479 + t6493 + t6526 + t6555;
    let t6558 = t449 * t6557;
    let t6568 = t1937 * t463;
    let t6569 = t1220 * t6568;
    let t6574 = -0.65854491829355115987e0_f64 * t446 * t6558 - 0.13170898365871023197e1_f64 * t1608 * t1659 - 0.13170898365871023197e1_f64 * t5365 + 0.13170898365871023197e1_f64 * t5369 + 0.26341796731742046394e1_f64 * t5372 + 0.13170898365871023197e1_f64 * t1215 * t1915 + 0.13170898365871023197e1_f64 * t446 * t6569 - t3932 + t5381 - 0.13170898365871023197e1_f64 * t5382 - t5388 - 0.65854491829355115987e0_f64 * t3935 + t3939;
    (t6557, t6558, t6569, t6574)
}
