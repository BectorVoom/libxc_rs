//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1766/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1766<F: Float>(t1196: F, t20472: F, t20671: F, t1188: F, t3495: F, t90352: F, t24498: F, t5192: F, t5184: F, t81310: F, t20400: F, t6548: F) -> (F, F, F, F, F) {
    let t90588 = F::cast_from(0.62337092780453269531e3_f64) * t1196 * t20472 * t20671;
    let t90592 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t3495 * t90352 * t1188;
    let t90594 = F::cast_from(0.23392894490538584828e1_f64) * t5192 * t24498;
    let t90597 = F::cast_from(0.69263436422725855036e2_f64) * t1196 * t81310 * t5184;
    let t90599 = F::cast_from(0.70178683471615754484e1_f64) * t20400 * t6548;
    (t90588, t90592, t90594, t90597, t90599)
}
