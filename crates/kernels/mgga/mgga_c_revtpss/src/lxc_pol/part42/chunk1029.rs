//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1029/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1029<F: Float>(t14036: F, t4018: F, t3989: F, t5629: F, t3930: F, t5661: F, t5665: F, t9976: F, t1412: F, t1882: F, t3938: F, t3992: F, t2661: F, t1399: F, t5608: F, t5651: F) -> (F, F, F, F, F, F, F, F) {
    let t14038 = 0.25410001404642664112e-4 * t4018 * t14036;
    let t14040 = 0.40015750243531754508e-1 * t3989 * t5629;
    let t14042 = 0.20007875121765877254e-2 * t3930 * t5661;
    let t14043 = t9976 * t5665;
    let t14045 = t1412 * t1882;
    let t14046 = t14045 * t3938;
    let t14047 = t3992 * t14046;
    let t14049 = 0.57165357490759649296e-4 * t2661 * t14047;
    let t14050 = t5608 * t1399;
    let t14051 = t3992 * t14050;
    let t14053 = 0.14291339372689912324e-4 * t2661 * t14051;
    let t14054 = t5651 * t1399;
    (t14038, t14040, t14042, t14043, t14045, t14049, t14053, t14054)
}
