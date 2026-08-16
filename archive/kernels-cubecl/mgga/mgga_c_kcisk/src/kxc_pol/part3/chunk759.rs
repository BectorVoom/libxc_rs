//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 759/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk759<F: Float>(t1906: F, t724: F, t1957: F, t5219: F, t5339: F, t5218: F, t10534: F, t5290: F, t5289: F, t10381: F, t7315: F, t10431: F) -> (F, F, F, F, F) {
    let t11699 = t1906 * t1906;
    let t11700 = F::cast_from(1.0_f64) / t11699;
    let t11701 = t724 * t11700;
    let t11702 = t5219 * t1957;
    let t11704 = F::cast_from(6.0_f64) * t11701 * t11702;
    let t11705 = t1957 * t5339;
    let t11707 = F::cast_from(6.0_f64) * t5218 * t11705;
    let t11708 = t5290 * t10534;
    let t11709 = t5289 * t11708;
    let t11711 = t5290 * t10381;
    let t11712 = t7315 * t11711;
    let t11714 = t5290 * t10431;
    (t11704, t11707, t11709, t11712, t11714)
}
