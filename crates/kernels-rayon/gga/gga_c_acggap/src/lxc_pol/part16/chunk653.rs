//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 653/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk653(t150: f64, t187: f64, t6413: f64, t1915: f64, t857: f64, t119: f64, t3862: f64, t3869: f64, t4107: f64, t4113: f64, t4123: f64, t464: f64, t5512: f64, t5514: f64, t5518: f64, t5520: f64, t5523: f64, t5525: f64) -> f64 {
    let t6415 = t6413 * t150 * t187;
    let t6418 = t857 * t1915;
    let t6421 = 0.65854491829355115987e0_f64 * t5512 + 0.13170898365871023197e1_f64 * t5514 + t4107 - 0.13170898365871023197e1_f64 * t5518 + t4113 - 0.65854491829355115987e0_f64 * t5520 * t464 - 0.13170898365871023197e1_f64 * t5523 - 0.65854491829355115987e0_f64 * t5525 + 0.65854491829355115987e0_f64 * t119 * t6415 + 0.13170898365871023197e1_f64 * t6418 + t3862 + t4123 + 0.13170898365871023197e1_f64 * t3869;
    t6421
}
