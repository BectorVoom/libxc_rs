//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1168/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1168<F: Float>(t20356: F, t1499: F, t7035: F, t16931: F, t16783: F, t16787: F, t16906: F, t16909: F, t16915: F, t16923: F, t20346: F, t20348: F, t20349: F, t20350: F, t20351: F, t20352: F, t20354: F) -> (F, F, F, F) {
    let t20357 = F::cast_from(0.17544670867903938621e1_f64) * t20356;
    let t20358 = t7035 * t1499;
    let t20359 = F::cast_from(0.17544670867903938621e1_f64) * t20358;
    let t20360 = F::new(48.0) * t16931;
    let t20361 = t16783 - t16787 - t20346 - t16906 + t16909 - t20348 + t20349 + t16915 - t20350 + t20351 - t16923 - t20352 - t20354 - t20357 - t20359 - t20360;
    (t20357, t20359, t20360, t20361)
}
