//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 915/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk915<F: Float>(t521: F, t9855: F, t2496: F, t4038: F, t123: F, t1330: F, t2630: F, t2516: F, t676: F, t3869: F, t3926: F, t3930: F, t221: F, t4019: F, t4057: F, t4018: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9856 = t9855 * t521;
    let t9858 = t4038 * t2496;
    let t9860 = t1330 * t123;
    let t9861 = t9860 * t2630;
    let t9863 = t676 * t2516;
    let t9865 = 0.16265371950452609763e-1 * t3869 * t9863;
    let t9866 = t676 * t2496;
    let t9868 = 0.48159733137676571078e0 * t3869 * t9866;
    let t9896 = t3930 * t3926;
    let t9905 = t4019 * t221 * t4057;
    let t9906 = t4018 * t9905;
    (t9856, t9858, t9861, t9863, t9865, t9866, t9868, t9896, t9906)
}
