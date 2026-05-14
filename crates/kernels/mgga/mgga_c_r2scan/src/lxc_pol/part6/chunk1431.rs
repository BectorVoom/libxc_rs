//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1431/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1431<F: Float>(t26948: F, t19405: F, t23802: F, t23803: F, t23829: F, t23831: F, t23835: F, t23837: F, t23895: F, t23896: F, t23897: F, t1783: F, t2483: F, t5879: F, t898: F, t6032: F, t7902: F) -> (F, F, F, F) {
    let t26949 = 0.4051561992e0 * t26948;
    let t26950 = -t23802 + t23803 - t19405 - t23829 + t23831 - t23835 - t23837 - t23895 + t26949 - t23896 + t23897;
    let t26952 = t2483 * t1783;
    let t26955 = t898 * t5879;
    let t26958 = t6032 * t7902;
    (t26950, t26952, t26955, t26958)
}
