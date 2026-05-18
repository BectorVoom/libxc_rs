//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 561/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk561<F: Float>(t1334: F, t8232: F, t1882: F, t5745: F, t5641: F, t5650: F, t1326: F, t463: F) -> (F, F, F, F, F) {
    let t23311 = F::new(4.0) / F::new(27.0) * t8232 * t1334;
    let t23312 = t1882 * t5745;
    let t23319 = t1882 * t5641;
    let t23321 = t1882 * t5650;
    let t23323 = t463 * t1326;
    (t23311, t23312, t23319, t23321, t23323)
}
