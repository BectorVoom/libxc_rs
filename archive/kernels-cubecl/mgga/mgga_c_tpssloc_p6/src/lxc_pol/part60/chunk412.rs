//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 412/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk412<F: Float>(t3862: F, t555: F, t1361: F, t835: F, t1336: F, t1995: F, t241: F, t67: F, t1376: F, t566: F) -> (F, F, F, F) {
    let t3864 = F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t555 * t3862;
    let t3865 = t1361 * t835;
    let t3866 = t1336 * t3865;
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3886 = F::cast_from(1.0_f64) / t1376 / t566;
    (t3864, t3866, t3870, t3886)
}
