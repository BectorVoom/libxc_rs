//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 688/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk688<F: Float>(t1990: F, t509: F, t1796: F, t1772: F, t603: F, t1994: F, t171: F, t1974: F, t2045: F, t592: F, t2020: F, t2029: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6745 = t509 * t1990;
    let t6747 = F::cast_from(0.32530742648344572643e-1_f64) * t1796 * t6745;
    let t6748 = t1772 * t603;
    let t6750 = F::cast_from(0.21687161765563048428e-1_f64) * t1796 * t6748;
    let t6751 = t509 * t1994;
    let t6753 = F::cast_from(0.48159446095139119799e0_f64) * t1796 * t6751;
    let t6766 = F::cast_from(1.0_f64) / t1974 / t171;
    let t6770 = t2045 * t592;
    let t6771 = F::cast_from(36.0_f64) * t6770;
    let t6799 = t2020 * t2029;
    (t6745, t6747, t6748, t6750, t6751, t6753, t6766, t6771, t6799)
}
