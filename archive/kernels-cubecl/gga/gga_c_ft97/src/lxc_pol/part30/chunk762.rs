//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 762/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk762<F: Float>(t2574: F, t265: F, t33307: F, t1882: F, t7504: F, t1456: F, t6079: F, t7543: F, t6061: F, t729: F, t1424: F, t6194: F) -> (F, F, F, F, F, F) {
    let t33626 = t2574 * t265 * t33307;
    let t33630 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t7504;
    let t33632 = t2574 * t1456 * t6079;
    let t33636 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1882 * t7543;
    let t33638 = t729 * t1456 * t6061;
    let t33642 = t729 * t6194 * t1424;
    (t33626, t33630, t33632, t33636, t33638, t33642)
}
