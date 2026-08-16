//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 953/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk953<F: Float>(t31206: F, t6897: F, t794: F, t1985: F, t1998: F, t214: F, t22870: F, t22716: F, t8480: F, t31203: F, t6914: F, t2006: F, t3791: F) -> (F, F, F, F, F) {
    let t114097 = t6897 * t794 * t31206;
    let t114098 = F::cast_from(0.16449340668482264365e-1_f64) * t114097;
    let t114102 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t214 * t1998 * t22870;
    let t114104 = F::cast_from(0.12793931631041761173e0_f64) * t22716 * t8480;
    let t114105 = t6914 * t31203;
    let t114106 = F::cast_from(0.76763589786250567036e-1_f64) * t114105;
    let t114107 = t2006 * t3791;
    (t114098, t114102, t114104, t114106, t114107)
}
