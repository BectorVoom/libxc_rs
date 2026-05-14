//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1231/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1231<F: Float>(t16476: F, t1535: F, t16283: F, t16287: F, t16290: F, t16481: F, t16486: F, t16489: F, t16493: F, t16497: F, t23916: F, t23917: F, t3396: F, t5082: F, t16498: F, t16500: F) -> (F, F, F, F) {
    let t23921 = 0.70178683471615754484e1 * t16476;
    let t23922 = -3.0 * t1535 * t3396 * t5082 + t16283 + t16287 - t16290 + t16481 - t16486 - t16489 - t16493 + t16497 - t23916 - t23917 + t23921;
    let t23924 = 0.5848223622634646207e0 * t16498;
    let t23925 = 0.17315859105681463759e2 * t16500;
    (t23921, t23922, t23924, t23925)
}
