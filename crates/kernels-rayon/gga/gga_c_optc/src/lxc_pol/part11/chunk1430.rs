//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1430/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1430(t1442: f64, t54837: f64, t15776: f64, t5101: f64, t1179: f64, t12606: f64, t12635: f64, t12860: f64, t15850: f64, t15856: f64, t15865: f64, t17988: f64, t18009: f64, t18117: f64, t18120: f64, t3103: f64, t3146: f64, t3244: f64, t3245: f64, t4289: f64, t46014: f64, t46172: f64, t5359: f64, t54797: f64, t54799: f64, t54843: f64, t54846: f64, t55768: f64, t58865: f64, t59618: f64, t894: f64) -> (f64, f64, f64) {
    let t59722 = t54837 * t1442;
    let t59731 = t15776 * t5101;
    let t59752 = 0.44430618325890501511e2_f64 * t46172 * t5359 + 0.15146801702008125515e1_f64 * t3244 * t3245 * t59722 + 0.35163949364965747848e4_f64 * t12606 * t46014 * t59618 - 0.80609127133382715662e-1_f64 * t54797 - 0.80782942410710002746e1_f64 * t54799 + 0.3029360340401625103e1_f64 * t3244 * t4289 * t59731 - 0.93770531639908660928e4_f64 * t15850 * t18117 + 0.46885265819954330464e4_f64 * t15856 * t18120 - 0.12117441361606500412e2_f64 * t12635 * t17988 + 0.33587136305576131525e-1_f64 * t1179 * t894 * t3146 * t58865 - 0.30972456242994093474e2_f64 * t3103 * t15865 * t55768 - 0.24951672488470492992e3_f64 * t12860 * t18009 + 0.11721316454988582616e4_f64 * t54843 - 0.41296608323992124631e2_f64 * t54846;
    (t59722, t59731, t59752)
}
