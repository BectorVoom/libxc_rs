//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1112/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1112<F: Float>(t2035: F, t3379: F, t7318: F, t136825: F, t32774: F, t34910: F, t32767: F, t34906: F, t1013: F, t32186: F, t52: F, t3404: F, t7182: F) -> (F, F, F, F, F) {
    let t147497 = t2035 * t7318 * t3379;
    let t147505 = t32774 * t136825 * t34910;
    let t147511 = t32767 * t136825 * t34906;
    let t147517 = t52 * t32186 * t1013;
    let t147521 = t52 * t7182 * t3404;
    (t147497, t147505, t147511, t147517, t147521)
}
