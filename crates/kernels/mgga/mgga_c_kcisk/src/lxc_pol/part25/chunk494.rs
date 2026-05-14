//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 494/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk494<F: Float>(t1900: F, t4581: F, t1869: F, t3517: F, t710: F, t1879: F, t3521: F, t1417: F, t1884: F, t1889: F, t579: F, t695: F) -> (F, F, F, F, F, F, F, F) {
    let t4582 = t4581 * t1900;
    let t4583 = t1869 * t4582;
    let t4586 = 0.21901432222222222222e-3 * t3517 * t710;
    let t4587 = t3521 * t1879;
    let t4589 = t1417 * t1884;
    let t4591 = t1417 * t1889;
    let t4593 = t579 * t695;
    let t4594 = 1.0 / t4593;
    (t4582, t4583, t4586, t4587, t4589, t4591, t4593, t4594)
}
