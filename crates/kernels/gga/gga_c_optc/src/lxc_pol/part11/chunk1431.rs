//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1431/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1431<F: Float>(t17336: F, t4477: F, t15776: F, t5110: F, t4570: F, t5328: F, t11596: F, t1179: F, t12042: F, t12617: F, t12860: F, t18013: F, t18025: F, t18114: F, t3103: F, t3151: F, t3234: F, t3244: F, t3245: F, t4450: F, t4457: F, t46152: F, t54850: F, t54853: F, t54904: F, t54911: F, t54926: F, t55145: F, t58865: F, t894: F, t8974: F) -> (F, F, F) {
    let t59756 = t17336 * t4477;
    let t59762 = t5110 * t15776;
    let t59766 = t4570 * t5328;
    let t59788 = -F::new(0.3029360340401625103e1) * t54850 + F::new(0.44430618325890501511e2) * t54853 + F::new(0.47123383072914168269e1) * t3244 * t11596 * t59756 - F::new(0.4158612081411748832e3) * t12860 * t18013 - F::new(0.45440405106024376544e1) * t3244 * t3245 * t59762 - F::new(0.35163949364965747848e4) * t4457 * t12617 * t59766 * t8974 + F::new(0.49903344976940985984e3) * t12860 * t18114 + F::new(0.69310201356862480534e2) * t3234 * t12042 * t59756 + F::new(0.18545411178216016757e1) * t4450 * t18025 - F::new(0.51620760404990155789e2) * t3103 * t46152 * t55145 - F::new(0.40304563566691357832e-1) * t1179 * t894 * t3151 * t58865 + F::new(0.15146801702008125515e1) * t54904 + F::new(0.20195735602677500687e1) * t54911 + F::new(0.6717427261115226305e-1) * t54926;
    (t59762, t59766, t59788)
}
