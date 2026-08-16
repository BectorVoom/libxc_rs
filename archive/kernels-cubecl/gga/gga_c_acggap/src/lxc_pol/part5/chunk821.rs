//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 821/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk821<F: Float>(t6479: F, t6493: F, t6526: F, t6555: F, t449: F, t1937: F, t463: F, t1220: F, t1215: F, t1608: F, t1659: F, t1915: F, t3932: F, t3935: F, t3939: F, t446: F, t5365: F, t5369: F, t5372: F, t5381: F, t5382: F, t5388: F) -> (F, F, F, F) {
    let t6557 = t6479 + t6493 + t6526 + t6555;
    let t6558 = t449 * t6557;
    let t6568 = t1937 * t463;
    let t6569 = t1220 * t6568;
    let t6574 = -F::cast_from(0.65854491829355115987e0_f64) * t446 * t6558 - F::cast_from(0.13170898365871023197e1_f64) * t1608 * t1659 - F::cast_from(0.13170898365871023197e1_f64) * t5365 + F::cast_from(0.13170898365871023197e1_f64) * t5369 + F::cast_from(0.26341796731742046394e1_f64) * t5372 + F::cast_from(0.13170898365871023197e1_f64) * t1215 * t1915 + F::cast_from(0.13170898365871023197e1_f64) * t446 * t6569 - t3932 + t5381 - F::cast_from(0.13170898365871023197e1_f64) * t5382 - t5388 - F::cast_from(0.65854491829355115987e0_f64) * t3935 + t3939;
    (t6557, t6558, t6569, t6574)
}
