//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 660/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk660<F: Float>(t182: F, t6413: F, t1251: F, t1839: F, t1925: F, t377: F, t1411: F, t1651: F, t119: F, t151: F, t3827: F, t4235: F, t4244: F, t4246: F, t6495: F, t6498: F, t6501: F, t6503: F, t6507: F, t6510: F, t6513: F) -> F {
    let t6515 = t182 * t6413;
    let t6518 = t1251 * t1839;
    let t6521 = t377 * t1925;
    let t6523 = t1651 * t1411;
    let t6526 = -F::cast_from(0.65854491829355115987e0_f64) * t151 * t6495 - F::cast_from(0.65854491829355115987e0_f64) * t6498 + F::cast_from(0.13170898365871023197e1_f64) * t4235 - F::cast_from(0.65854491829355115987e0_f64) * t6501 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t6503 - t4244 + F::cast_from(0.26341796731742046394e1_f64) * t4246 - t3827 - F::cast_from(0.13170898365871023197e1_f64) * t151 * t6507 + F::cast_from(0.13170898365871023197e1_f64) * t151 * t6510 + F::cast_from(0.65854491829355115987e0_f64) * t6513 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t6515 - F::cast_from(0.65854491829355115987e0_f64) * t151 * t6518 - F::cast_from(0.13170898365871023197e1_f64) * t6521 - F::cast_from(0.13170898365871023197e1_f64) * t151 * t6523;
    t6526
}
