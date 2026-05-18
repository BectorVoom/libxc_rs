//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 537/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk537<F: Float>(t218: F, t219: F, t2226: F, t2185: F, t334: F, t2175: F, t2187: F, t2205: F, t2210: F, t2212: F, t2216: F, t2218: F, t2222: F, t2224: F) -> (F, F, F, F) {
    let t2228 = t218 * t219 * t2226;
    let t2230 = t334 * t2185;
    let t2232 = t218 * t219 * t2230;
    let t2234 = -F::new(0.9494625e0) * t2205 + F::new(0.1898925e1) * t2210 + t2212 - F::new(0.59793333333333333334e0) * t2175 + F::new(0.8969e0) * t2187 + F::new(0.15358125e0) * t2216 + F::new(0.3071625e0) * t2218 + t2222 - F::new(0.32862666666666666666e0) * t2224 + F::new(0.24647e0) * t2228 + F::new(0.24647e0) * t2232;
    (t2228, t2230, t2232, t2234)
}
