//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1004/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1004<F: Float>(t35475: F, t1992: F, t7585: F, t7586: F, t8906: F, t1983: F, t8402: F, t30105: F, t8897: F, t30268: F, t8783: F, t1479: F, t429: F) -> (F, F, F, F, F, F) {
    let t35476 = F::cast_from(0.7145669686344956162e-3_f64) * t35475;
    let t35479 = t7585 * t7586 * t1992 * t8906;
    let t35480 = F::cast_from(0.28582678745379824648e-3_f64) * t35479;
    let t35484 = t7585 * t7586 * t1983 * t8402;
    let t35485 = F::cast_from(0.14291339372689912324e-3_f64) * t35484;
    let t35486 = t30105 * t8897;
    let t35496 = t30268 * t8783;
    let t35497 = F::cast_from(0.94344276868812456204e-2_f64) * t35496;
    let t35500 = t429 * t1479;
    (t35476, t35480, t35485, t35486, t35497, t35500)
}
