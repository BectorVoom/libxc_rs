//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 925/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk925<F: Float>(t2684: F, t43486: F, t7585: F, t10930: F, t10931: F, t23220: F, t43598: F, t43683: F, t7572: F, t7573: F, t43494: F, t7427: F) -> (F, F, F, F, F) {
    let t43793 = F::cast_from(0.87421871174939309262e2_f64) * t2684 * t7585 * t43486;
    let t43800 = F::cast_from(0.55213813373645879534e2_f64) * t10930 * t10931 * t43486;
    let t43803 = F::cast_from(0.27606906686822939767e2_f64) * t23220 * t10931 * t43598;
    let t43806 = F::cast_from(0.69017266717057349418e1_f64) * t7572 * t7573 * t43683;
    let t43809 = F::cast_from(0.37959496694381542179e3_f64) * t7427 * t7573 * t43494;
    (t43793, t43800, t43803, t43806, t43809)
}
