//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 898/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk898<F: Float>(t136159: F, t136188: F, t25888: F, t32069: F, t136189: F, t137245: F, t26016: F, t34482: F, t358: F, t1564: F, t363: F, t446: F, t1882: F, t34488: F, t18: F, t32333: F, t3281: F, t7824: F) -> (F, F, F, F, F) {
    let t144908 = t136159 * t136188 * t32069 * t25888;
    let t144912 = t136159 * t137245 * t136189 * t26016;
    let t144914 = t34482 * t358;
    let t144917 = t446 * t1564 * t144914 * t363;
    let t144919 = t1882 * t34488;
    let t144923 = t3281 * t7824 * t32333 * t18;
    (t144908, t144912, t144917, t144919, t144923)
}
