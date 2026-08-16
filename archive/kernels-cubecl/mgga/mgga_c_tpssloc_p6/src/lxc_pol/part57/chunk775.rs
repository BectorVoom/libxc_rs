//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 775/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk775<F: Float>(t1527: F, t7537: F, t2718: F, t1911: F, t5636: F, t10110: F, t5657: F, t16815: F, t232: F, t6646: F, t1888: F, t5544: F, t6638: F) -> (F, F, F, F, F, F, F, F) {
    let t28306 = t7537 * t1527;
    let t28307 = t2718 * t28306;
    let t28310 = t1911 * t5636;
    let t28311 = t10110 * t28310;
    let t28316 = t1911 * t5657;
    let t28317 = t2718 * t28316;
    let t28321 = t16815 * t232;
    let t28322 = t6646 * t28321;
    let t28323 = t1888 * t28322;
    let t28329 = t6638 * t5544;
    (t28306, t28307, t28310, t28311, t28316, t28317, t28323, t28329)
}
