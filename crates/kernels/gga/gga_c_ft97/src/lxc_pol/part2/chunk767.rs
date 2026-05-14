//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 767/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk767<F: Float>(t13624: F, t701: F, t3813: F, t8715: F, t2917: F, t668: F, t228: F, t9634: F, t2436: F, t3799: F, t2452: F, t2443: F, t1103: F, t231: F, t625: F, t2448: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13625 = t701 * t13624;
    let t13628 = t8715 * t3813;
    let t13629 = t701 * t13628;
    let t13631 = t2917 * t668;
    let t13633 = t228 * t9634 * t13631;
    let t13635 = t3799 * t2436;
    let t13636 = 0.1134997482304526749e-1 * t13635;
    let t13637 = t3799 * t2452;
    let t13639 = t3799 * t2443;
    let t13643 = t228 * t1103 * t625 * t231;
    let t13645 = t3799 * t2448;
    (t13625, t13629, t13633, t13635, t13636, t13637, t13639, t13643, t13645)
}
