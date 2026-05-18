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
    let t28174 = -F::new(0.92835860883789062501e-5) * t27077 * t28137 - F::new(0.11584201388888888889e-3) * t27014 * t8091 - F::new(0.23168402777777777778e-3) * t7788 * t28146 + F::new(0.11607361111111111111e-2) * t27838 + F::new(0.15459116753472222222e-4) * t26955 * t28102 + F::new(0.34752604166666666667e-3) * t7788 * t28153 + t26974 + F::new(0.11607361111111111111e-2) * t27843 + F::new(0.23214722222222222222e-2) * t27847 - F::new(0.92673611111111111112e-3) * t26966 * t8095 + F::new(0.11584201388888888889e-3) * t28161 + F::new(0.77382407407407407407e-3) * t27849 - F::new(0.30952962962962962963e-2) * t27852 + F::new(0.11607361111111111111e-2) * t27860 - F::new(0.11607361111111111111e-2) * t27865 + F::new(0.77382407407407407407e-3) * t27868 - F::new(0.23214722222222222222e-2) * t27871 + F::new(0.19345601851851851852e-2) * t27874 - F::new(0.11607361111111111111e-2) * t27877 + F::new(0.92673611111111111112e-3) * t28171 * t2197;
    (t28160, t28171, t28174)
}
