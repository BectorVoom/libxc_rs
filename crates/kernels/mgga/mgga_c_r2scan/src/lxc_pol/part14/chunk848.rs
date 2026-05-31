//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 848/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk848<F: Float>(t1416: F, t959: F, t5237: F, t7647: F, t7650: F, t7653: F, t7656: F, t7659: F, t7661: F, t7662: F, t7664: F, t7667: F, t7669: F) -> F {
    let t7671 = t1416 * t959;
    let t7673 = -F::cast_from(0.21687162600603479684e-1_f64) * t5237 + F::cast_from(0.19263893255070628431e1_f64) * t7647 + F::cast_from(0.1714584e0_f64) * t7650 - t7653 + t7656 + t7659 + t7661 - F::cast_from(0.10389515463408878255e3_f64) * t7662 + F::cast_from(0.35089341735807877242e1_f64) * t7664 - F::cast_from(0.33872559466666666666e-2_f64) * t7667 + F::cast_from(0.72290542002011598948e-2_f64) * t7669 - F::cast_from(20.0_f64) * t7671;
    t7673
}
