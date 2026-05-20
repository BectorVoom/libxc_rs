//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2480/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2480<F: Float>(t48614: F, t14005: F, t46740: F, t46917: F, t5697: F, t14036: F, t9976: F, t46694: F, t5686: F, t13769: F, t808: F, t9736: F) -> (F, F, F, F, F, F) {
    let t48615 = F::cast_from(0.17006693853500995666e-1_f64) * t48614;
    let t48637 = t46740 * t14005;
    let t48638 = F::cast_from(0.40656002247428262579e-3_f64) * t48637;
    let t48645 = t46917 * t5697;
    let t48668 = t9976 * t14036;
    let t48669 = F::cast_from(0.40656002247428262579e-3_f64) * t48668;
    let t48685 = t46694 * t5686;
    let t48686 = F::new(35.0) / F::new(24.0) * t48685;
    let t48690 = t9736 * t808 * t13769;
    (t48615, t48638, t48645, t48669, t48686, t48690)
}
