//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 634/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk634<F: Float>(t265: F, t504: F, t1238: F, t2121: F, t2124: F, t2145: F, t2155: F, t498: F, t1256: F, t193: F, t1964: F, t336: F) -> (F, F) {
    let t505 = t265 < t504;
    let t2157 = F::cast_from(0.82246703342411321825e-2_f64) * t2121 * t2124 + t2145 * t498 - t1238 * t2155;
    let t2161 = piecewise3::<F>(t505, t193 * t336 * t2157 * t1256, t1964);
    (t2157, t2161)
}
