//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 863/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk863<F: Float>(t1882: F, t34104: F, t7669: F, t8232: F, t34164: F, t34111: F, t34146: F, t34150: F, t34099: F, t2842: F, t7679: F, t34083: F, t8392: F, t34095: F, t34178: F, t34183: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t144087 = t1882 * t34104;
    let t144093 = 8.0 / 27.0 * t8232 * t7669;
    let t144094 = t1882 * t34164;
    let t144096 = t1882 * t34111;
    let t144105 = t1882 * t34146;
    let t144107 = t1882 * t34150;
    let t144123 = t1882 * t34099;
    let t144131 = t2842 * t7679;
    let t144140 = t8392 * t34083;
    let t144142 = t1882 * t34095;
    let t144148 = t1882 * t34178;
    let t144150 = t1882 * t34183;
    (t144087, t144093, t144094, t144096, t144105, t144107, t144123, t144131, t144140, t144142, t144148, t144150)
}
