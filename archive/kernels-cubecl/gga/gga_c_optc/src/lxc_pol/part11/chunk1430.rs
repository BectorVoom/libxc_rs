//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1430/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1430<F: Float>(t1442: F, t54837: F, t15776: F, t5101: F, t1179: F, t12606: F, t12635: F, t12860: F, t15850: F, t15856: F, t15865: F, t17988: F, t18009: F, t18117: F, t18120: F, t3103: F, t3146: F, t3244: F, t3245: F, t4289: F, t46014: F, t46172: F, t5359: F, t54797: F, t54799: F, t54843: F, t54846: F, t55768: F, t58865: F, t59618: F, t894: F) -> (F, F, F) {
    let t59722 = t54837 * t1442;
    let t59731 = t15776 * t5101;
    let t59752 = F::cast_from(0.44430618325890501511e2_f64) * t46172 * t5359 + F::cast_from(0.15146801702008125515e1_f64) * t3244 * t3245 * t59722 + F::cast_from(0.35163949364965747848e4_f64) * t12606 * t46014 * t59618 - F::cast_from(0.80609127133382715662e-1_f64) * t54797 - F::cast_from(0.80782942410710002746e1_f64) * t54799 + F::cast_from(0.3029360340401625103e1_f64) * t3244 * t4289 * t59731 - F::cast_from(0.93770531639908660928e4_f64) * t15850 * t18117 + F::cast_from(0.46885265819954330464e4_f64) * t15856 * t18120 - F::cast_from(0.12117441361606500412e2_f64) * t12635 * t17988 + F::cast_from(0.33587136305576131525e-1_f64) * t1179 * t894 * t3146 * t58865 - F::cast_from(0.30972456242994093474e2_f64) * t3103 * t15865 * t55768 - F::cast_from(0.24951672488470492992e3_f64) * t12860 * t18009 + F::cast_from(0.11721316454988582616e4_f64) * t54843 - F::cast_from(0.41296608323992124631e2_f64) * t54846;
    (t59722, t59731, t59752)
}
