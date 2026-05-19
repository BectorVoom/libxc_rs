//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 876/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk876<F: Float>(t10914: F, t10915: F, t45320: F, t45305: F, t7584: F, t7585: F, t10930: F, t10931: F, t45423: F, t11765: F, t2714: F, t7572: F, t7573: F) -> (F, F, F, F, F) {
    let t45633 = F::cast_from(0.42900587942220512002e1_f64) * t10914 * t10915 * t45320;
    let t45636 = F::cast_from(0.11502877786176224903e2_f64) * t7584 * t7585 * t45305;
    let t45639 = F::cast_from(0.27606906686822939767e2_f64) * t10930 * t10931 * t45423;
    let t45648 = F::cast_from(0.35750489951850426669e0_f64) * t2714 * t11765;
    let t45653 = F::cast_from(0.69017266717057349418e1_f64) * t7572 * t7573 * t45305;
    (t45633, t45636, t45639, t45648, t45653)
}
