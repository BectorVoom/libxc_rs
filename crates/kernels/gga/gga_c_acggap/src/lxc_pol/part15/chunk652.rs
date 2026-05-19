//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 652/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk652<F: Float>(t150: F, t187: F, t6413: F, t1915: F, t857: F, t119: F, t3862: F, t3869: F, t4107: F, t4113: F, t4123: F, t464: F, t5512: F, t5514: F, t5518: F, t5520: F, t5523: F, t5525: F) -> F {
    let t6415 = t6413 * t150 * t187;
    let t6418 = t857 * t1915;
    let t6421 = F::cast_from(0.65854491829355115987e0_f64) * t5512 + F::cast_from(0.13170898365871023197e1_f64) * t5514 + t4107 - F::cast_from(0.13170898365871023197e1_f64) * t5518 + t4113 - F::cast_from(0.65854491829355115987e0_f64) * t5520 * t464 - F::cast_from(0.13170898365871023197e1_f64) * t5523 - F::cast_from(0.65854491829355115987e0_f64) * t5525 + F::cast_from(0.65854491829355115987e0_f64) * t119 * t6415 + F::cast_from(0.13170898365871023197e1_f64) * t6418 + t3862 + t4123 + F::cast_from(0.13170898365871023197e1_f64) * t3869;
    t6421
}
