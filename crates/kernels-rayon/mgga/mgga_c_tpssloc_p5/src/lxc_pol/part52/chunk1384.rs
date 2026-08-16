//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1384/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1384(t120092: f64, t120095: f64, t120097: f64, t120103: f64, t120104: f64, t120107: f64, t123178: f64, t123180: f64, t123182: f64, t123184: f64, t123187: f64, t123189: f64, t5361: f64, t8687: f64) -> f64 {
    let t123191 = t5361 * t8687 - t120092 + t120095 - t120097 + t120103 - 3.0_f64 * t120104 + t120107 - 3.0_f64 * t123178 - 2.0_f64 * t123180 - 2.0_f64 * t123182 - 2.0_f64 * t123184 - 2.0_f64 * t123187 + t123189;
    t123191
}
