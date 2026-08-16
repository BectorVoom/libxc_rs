//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1024/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1024<F: Float>(t1527: F, t4300: F, t2718: F, t17050: F, t17052: F, t17057: F, t17060: F, t17064: F, t259: F, t2597: F, t2713: F, t4147: F, t4268: F, t4273: F, t4301: F, t5637: F, t5658: F, t855: F, t866: F) -> F {
    let t17069 = t1527 * t4300;
    let t17070 = t2718 * t17069;
    let t17079 = -t17050 * t855 - t17052 * t866 + F::cast_from(2.0_f64) * t17057 * t855 + t17060 * t259 - F::cast_from(6.0_f64) * t17064 * t855 + F::cast_from(4.0_f64) * t17070 * t855 + F::cast_from(2.0_f64) * t2597 * t5637 + F::cast_from(2.0_f64) * t2713 * t5637 - t2713 * t5658 + F::cast_from(4.0_f64) * t4147 * t4273 - F::cast_from(2.0_f64) * t4147 * t4301 - F::cast_from(2.0_f64) * t4268 * t4301;
    t17079
}
