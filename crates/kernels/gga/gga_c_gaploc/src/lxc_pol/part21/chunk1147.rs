//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1147/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1147<F: Float>(t2877: F, t6866: F, t6773: F, t2437: F, t8072: F, t10144: F, t4614: F, t597: F, t188: F, t31793: F, t3377: F, t8158: F, t9333: F, t1508: F, t2765: F, t524: F, t7930: F) -> (F, F, F, F, F, F, F, F) {
    let t34121 = 0.35750489951850426669e0 * t6866 * t2877;
    let t34123 = 0.71500979903700853338e0 * t6773 * t2877;
    let t34125 = 0.71500979903700853338e0 * t2437 * t8072;
    let t34128 = 0.30674340763136599742e2 * t597 * t4614 * t10144;
    let t34143 = 0.10725146985555128001e1 * t188 * t31793 * t3377;
    let t34145 = 0.21450293971110256002e1 * t8158 * t9333;
    let t34148 = 0.10725146985555128001e1 * t1508 * t2765 * t3377;
    let t34151 = 0.21450293971110256002e1 * t524 * t7930 * t3377;
    (t34121, t34123, t34125, t34128, t34143, t34145, t34148, t34151)
}
