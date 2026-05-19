//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 557/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk557<F: Float>(t2172: F, t2221: F, t2175: F, t2187: F, t2205: F, t2210: F, t2216: F, t2218: F, t2224: F, t2228: F, t2232: F) -> (F, F, F) {
    let t2303 = F::cast_from(0.40256666666666666667e0_f64) * t2172;
    let t2308 = F::new(0.137975e0) * t2221;
    let t2312 = -F::new(0.1294625e1) * t2205 + F::new(0.258925e1) * t2210 + t2303 - F::new(0.60385e0) * t2175 + F::new(0.905775e0) * t2187 + F::new(0.82524375e-1) * t2216 + F::new(0.16504875e0) * t2218 + t2308 - F::new(0.33114e0) * t2224 + F::new(0.248355e0) * t2228 + F::new(0.248355e0) * t2232;
    (t2303, t2308, t2312)
}
