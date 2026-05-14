//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1230/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1230<F: Float>(t19620: F, t19623: F, t19625: F, t19627: F, t16193: F, t16230: F, t16273: F, t16275: F, t16280: F, t19704: F, t2718: F, t2719: F, t6853: F, t9103: F, t16291: F, t19687: F) -> (F, F, F, F, F, F, F) {
    let t23906 = 0.11393789434848516923e-2 * t19620;
    let t23907 = 0.43374325201206959368e-1 * t19623;
    let t23908 = 0.32530743900905219526e-1 * t19625;
    let t23909 = 0.96319466275353142155e0 * t19627;
    let t23915 = 12.0 * t2718 * t2719 * t6853 + 24.0 * t19704 * t9103 - t16193 - t16230 - t16273 + t16275 - t16280 - t23906 + t23907 + t23908 + t23909;
    let t23916 = 8.0 * t16291;
    let t23917 = 48.0 * t19687;
    (t23906, t23907, t23908, t23909, t23915, t23916, t23917)
}
