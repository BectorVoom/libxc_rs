//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 926/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk926<F: Float>(t1253: F, t4075: F, t4083: F, t1229: F, t4030: F, t1254: F, t13589: F, t1255: F, t370: F, t4125: F, t13562: F, t4129: F) -> (F, F, F, F, F) {
    let t13702 = t4075 * t4083 * t1253;
    let t13705 = t1229 * t4030;
    let t13708 = t13589 * t1254;
    let t13711 = t1255 * t4075;
    let t13715 = F::cast_from(1.0_f64) / t4125 / t370;
    let t13717 = t13715 * t13562 * t4129;
    (t13702, t13705, t13708, t13711, t13717)
}
