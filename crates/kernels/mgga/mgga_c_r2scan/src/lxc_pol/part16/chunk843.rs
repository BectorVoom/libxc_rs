//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 843/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk843<F: Float>(t3128: F, t585: F, t159: F, t617: F, t5331: F, t5335: F, t5338: F, t5340: F, t5344: F, t5346: F, t5350: F, t5354: F, t5355: F, t7708: F) -> F {
    let t8915 = t3128 * t585;
    let t8916 = t159 * t8915;
    let t8917 = t8916 * t617;
    let t8925 = F::cast_from(0.84681398666666666666e-3_f64) * t8917 + F::new(16.0) * t7708 - t5331 + t5335 - F::cast_from(0.23392894490538584828e1_f64) * t5338 + F::cast_from(0.34631718211362927518e2_f64) * t5340 + F::cast_from(0.35089341735807877242e1_f64) * t5344 - F::cast_from(0.10389515463408878255e3_f64) * t5346 - t5350 - t5354 - F::cast_from(0.11696447245269292414e1_f64) * t5355;
    t8925
}
