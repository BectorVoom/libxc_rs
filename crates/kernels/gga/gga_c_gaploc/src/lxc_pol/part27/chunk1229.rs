//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1229/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1229<F: Float>(t10961: F, t2197: F, t10713: F, t4614: F, t833: F, t24364: F, t955: F, t16136: F, t3504: F, t28387: F, t3025: F, t10627: F, t1865: F) -> (F, F, F, F, F, F) {
    let t32771 = F::new(0.30674340763136599742e2) * t2197 * t10961;
    let t32774 = F::new(0.30674340763136599742e2) * t833 * t4614 * t10713;
    let t32778 = F::new(0.79445533226334281487e-1) * t955 * t24364;
    let t32785 = F::new(0.69017266717057349418e1) * t16136 * t3504;
    let t32791 = F::new(0.10725146985555128001e1) * t3025 * t28387;
    let t32803 = t10627 * t1865;
    (t32771, t32774, t32778, t32785, t32791, t32803)
}
