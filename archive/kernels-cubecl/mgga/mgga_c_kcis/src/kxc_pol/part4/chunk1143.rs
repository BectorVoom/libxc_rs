//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1143/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1143<F: Float>(t1662: F, t3040: F, t2894: F, t2909: F, t4972: F, t1003: F, t417: F, t1245: F, t4967: F, t991: F, t1704: F, t2911: F, t9874: F) -> (F, F, F, F) {
    let t14501 = t1662 * t3040;
    let t14502 = t2894 * t14501;
    let t14511 = t2909 * t4972;
    let t14512 = t14511 * t1003;
    let t14513 = t417 * t14512;
    let t14516 = t1245 * t4967;
    let t14518 = t991 * t14516 / F::cast_from(72.0_f64);
    let t14522 = t9874 * t1704 * t2911;
    (t14502, t14513, t14518, t14522)
}
