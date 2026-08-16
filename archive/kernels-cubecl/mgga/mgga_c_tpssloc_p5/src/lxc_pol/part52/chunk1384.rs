//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1384/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1384<F: Float>(t120092: F, t120095: F, t120097: F, t120103: F, t120104: F, t120107: F, t123178: F, t123180: F, t123182: F, t123184: F, t123187: F, t123189: F, t5361: F, t8687: F) -> F {
    let t123191 = t5361 * t8687 - t120092 + t120095 - t120097 + t120103 - F::cast_from(3.0_f64) * t120104 + t120107 - F::cast_from(3.0_f64) * t123178 - F::cast_from(2.0_f64) * t123180 - F::cast_from(2.0_f64) * t123182 - F::cast_from(2.0_f64) * t123184 - F::cast_from(2.0_f64) * t123187 + t123189;
    t123191
}
