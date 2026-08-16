//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1431/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1431(t17336: f64, t4477: f64, t15776: f64, t5110: f64, t4570: f64, t5328: f64, t11596: f64, t1179: f64, t12042: f64, t12617: f64, t12860: f64, t18013: f64, t18025: f64, t18114: f64, t3103: f64, t3151: f64, t3234: f64, t3244: f64, t3245: f64, t4450: f64, t4457: f64, t46152: f64, t54850: f64, t54853: f64, t54904: f64, t54911: f64, t54926: f64, t55145: f64, t58865: f64, t894: f64, t8974: f64) -> (f64, f64, f64) {
    let t59756 = t17336 * t4477;
    let t59762 = t5110 * t15776;
    let t59766 = t4570 * t5328;
    let t59788 = -0.3029360340401625103e1_f64 * t54850 + 0.44430618325890501511e2_f64 * t54853 + 0.47123383072914168269e1_f64 * t3244 * t11596 * t59756 - 0.4158612081411748832e3_f64 * t12860 * t18013 - 0.45440405106024376544e1_f64 * t3244 * t3245 * t59762 - 0.35163949364965747848e4_f64 * t4457 * t12617 * t59766 * t8974 + 0.49903344976940985984e3_f64 * t12860 * t18114 + 0.69310201356862480534e2_f64 * t3234 * t12042 * t59756 + 0.18545411178216016757e1_f64 * t4450 * t18025 - 0.51620760404990155789e2_f64 * t3103 * t46152 * t55145 - 0.40304563566691357832e-1_f64 * t1179 * t894 * t3151 * t58865 + 0.15146801702008125515e1_f64 * t54904 + 0.20195735602677500687e1_f64 * t54911 + 0.6717427261115226305e-1_f64 * t54926;
    (t59762, t59766, t59788)
}
