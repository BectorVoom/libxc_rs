//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 936/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk936<F: Float>(t24543: F, t33343: F, t1434: F, t33328: F, t681: F, t33324: F, t173: F, t24274: F, t33366: F, t6037: F, t32237: F, t33432: F, t3771: F) -> (F, F, F, F, F, F) {
    let t140843 = t24543 * t33343;
    let t140857 = t1434 * t681 * t33328;
    let t140863 = t1434 * t681 * t33324;
    let t140869 = t24274 * t173;
    let t140871 = t33366 * t140869 * t6037;
    let t140884 = t3771 * t33432 * t32237;
    (t140843, t140857, t140863, t140869, t140871, t140884)
}
