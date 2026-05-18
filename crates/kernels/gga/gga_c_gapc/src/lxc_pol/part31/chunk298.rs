//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 298/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk298<F: Float>(t1092: F, t1093: F, t1070: F, t1075: F, t1079: F, t1082: F, t1090: F) -> (F, F) {
    let t1094 = t1092 * t1093;
    let t1096 = F::new(0.13900948042322754167e-2) * t1070 + F::new(0.50602213541666666669e-5) * t1075 - F::new(0.86880925264517213544e-4) * t1079 - F::new(0.11594181388521408695e-4) * t1082 - F::new(0.42205124476153752644e-7) * t1090 + F::new(0.72463633678258804342e-6) * t1094;
    (t1094, t1096)
}
