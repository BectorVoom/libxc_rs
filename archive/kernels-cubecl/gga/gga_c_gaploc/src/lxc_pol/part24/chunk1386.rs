//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1386/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1386<F: Float>(t10601: F, t4372: F, t107: F, t31730: F, t544: F, t10392: F, t17568: F, t31557: F, t475: F) -> (F, F, F, F) {
    let t34556 = F::cast_from(0.92686455430723328401e-1_f64) * t10601 * t4372;
    let t34558 = t544 * t31730 * t107;
    let t34566 = F::cast_from(0.15337170381568299871e1_f64) * t17568 * t10392;
    let t34567 = t31557 * t475;
    (t34556, t34558, t34566, t34567)
}
