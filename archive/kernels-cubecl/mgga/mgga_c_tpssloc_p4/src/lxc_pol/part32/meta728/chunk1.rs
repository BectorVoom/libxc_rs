//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2366/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2366<F: Float>(t1851: F, t8119: F, t103103: F, t105102: F, t105115: F, t105128: F, t1396: F, t1398: F, t1404: F, t1852: F, t20149: F, t2174: F, t27930: F, t29866: F, t29884: F, t3: F, t5364: F, t580: F, t6483: F, t7416: F, t96281: F, t96283: F, t96285: F) -> F {
    let t105131 = t1851 * t8119;
    let t105139 = t103103 + t20149 * t2174 + F::cast_from(2.0_f64) * t5364 * t8119 + t29866 * t1404 + t96281 + t1398 * (t105115 + t105128) + F::cast_from(2.0_f64) * t105131 + t96283 + t7416 * t6483 + t3 * t105102 * t580 + F::cast_from(2.0_f64) * t1852 * t27930 + t1396 * t29884 + t96285;
    t105139
}
