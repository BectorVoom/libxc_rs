//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1163/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1163<F: Float>(t15573: F, t8094: F, t7788: F, t5345: F, t7779: F, t2197: F, t26955: F, t26966: F, t26974: F, t27014: F, t27077: F, t27838: F, t27843: F, t27847: F, t27849: F, t27852: F, t27860: F, t27865: F, t27868: F, t27871: F, t27874: F, t27877: F, t28102: F, t28137: F, t28146: F, t28153: F, t8091: F, t8095: F) -> (F, F, F) {
    let t28160 = t15573 * t8094;
    let t28161 = t7788 * t28160;
    let t28171 = t5345 * t7779;
    let t28174 = -F::cast_from(0.92835860883789062501e-5_f64) * t27077 * t28137 - F::cast_from(0.11584201388888888889e-3_f64) * t27014 * t8091 - F::cast_from(0.23168402777777777778e-3_f64) * t7788 * t28146 + F::cast_from(0.11607361111111111111e-2_f64) * t27838 + F::cast_from(0.15459116753472222222e-4_f64) * t26955 * t28102 + F::cast_from(0.34752604166666666667e-3_f64) * t7788 * t28153 + t26974 + F::cast_from(0.11607361111111111111e-2_f64) * t27843 + F::cast_from(0.23214722222222222222e-2_f64) * t27847 - F::cast_from(0.92673611111111111112e-3_f64) * t26966 * t8095 + F::cast_from(0.11584201388888888889e-3_f64) * t28161 + F::cast_from(0.77382407407407407407e-3_f64) * t27849 - F::cast_from(0.30952962962962962963e-2_f64) * t27852 + F::cast_from(0.11607361111111111111e-2_f64) * t27860 - F::cast_from(0.11607361111111111111e-2_f64) * t27865 + F::cast_from(0.77382407407407407407e-3_f64) * t27868 - F::cast_from(0.23214722222222222222e-2_f64) * t27871 + F::cast_from(0.19345601851851851852e-2_f64) * t27874 - F::cast_from(0.11607361111111111111e-2_f64) * t27877 + F::cast_from(0.92673611111111111112e-3_f64) * t28171 * t2197;
    (t28160, t28171, t28174)
}
