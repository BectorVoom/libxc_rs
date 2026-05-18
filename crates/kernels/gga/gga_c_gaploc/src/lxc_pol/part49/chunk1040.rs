//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1040/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1040<F: Float>(t10931: F, t23220: F, t43598: F, t43683: F, t7572: F, t7573: F, t43494: F, t7427: F, t10667: F, t2033: F, t2365: F, t2610: F) -> (F, F, F, F) {
    let t43803 = F::new(0.27606906686822939767e2) * t23220 * t10931 * t43598;
    let t43806 = F::new(0.69017266717057349418e1) * t7572 * t7573 * t43683;
    let t43809 = F::new(0.37959496694381542179e3) * t7427 * t7573 * t43494;
    let t43812 = t2033 * t2365 * t2610 * t10667;
    (t43803, t43806, t43809, t43812)
}
