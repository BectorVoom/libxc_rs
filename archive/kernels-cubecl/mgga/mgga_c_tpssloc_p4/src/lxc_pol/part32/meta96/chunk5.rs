//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 618/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk618<F: Float>(t25: F, t1918: F, t2116: F, t40: F, t337: F, t50: F, t1887: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t2119 = piecewise3::<F>(t115, t1918, t2116 * t40 / F::cast_from(2.0_f64));
    let t2120 = t50 * t337;
    let t2121 = t2120 * t1887;
    (t2119, t2120, t2121)
}
