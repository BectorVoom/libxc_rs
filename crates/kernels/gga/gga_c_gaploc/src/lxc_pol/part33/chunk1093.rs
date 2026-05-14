//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1093/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1093<F: Float>(t32760: F, t11038: F, t4614: F, t813: F, t10964: F, t2194: F, t10717: F, t833: F, t10961: F, t2197: F, t10713: F, t24364: F, t955: F, t16136: F, t3504: F, t28387: F, t3025: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32761 = 0.51123901271894332902e0 * t32760;
    let t32764 = 0.12269736305254639897e2 * t813 * t4614 * t11038;
    let t32766 = 0.12269736305254639897e2 * t2194 * t10964;
    let t32769 = 0.30674340763136599742e2 * t833 * t4614 * t10717;
    let t32771 = 0.30674340763136599742e2 * t2197 * t10961;
    let t32774 = 0.30674340763136599742e2 * t833 * t4614 * t10713;
    let t32778 = 0.79445533226334281487e-1 * t955 * t24364;
    let t32785 = 0.69017266717057349418e1 * t16136 * t3504;
    let t32791 = 0.10725146985555128001e1 * t3025 * t28387;
    (t32761, t32764, t32766, t32769, t32771, t32774, t32778, t32785, t32791)
}
