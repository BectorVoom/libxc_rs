//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 681/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk681<F: Float>(t493: F, t6766: F, t2541: F, t529: F, t2991: F, t2648: F, t443: F, t332: F, t1385: F, t439: F, t1908: F, t2002: F, t4161: F, t4162: F, t4165: F, t6733: F, t6738: F, t6740: F, t6743: F, t6746: F, t6750: F, t6754: F, t6758: F, t6763: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6768 = t493 * t6766 / 27.0;
    let t6769 = t2541 * t529;
    let t6770 = t2991 * t6769;
    let t6772 = t493 * t6770 / 27.0;
    let t6773 = t2648 * t443;
    let t6774 = t6773 * t332;
    let t6775 = t1385 * t6774;
    let t6777 = t439 * t6775 / 45.0;
    let t6779 = 2.0 / 45.0 * t2002 * t1908;
    let t6780 = -t6733 - t6738 - t6740 - t4161 + 0.033245444444444446 * t4162 + t4165 - t6743 - t6746 - t6750 + t6754 - t6758 - t6763 + t6768 - t6772 - t6777 - t6779;
    (t6768, t6769, t6770, t6772, t6773, t6774, t6775, t6777, t6779, t6780)
}
