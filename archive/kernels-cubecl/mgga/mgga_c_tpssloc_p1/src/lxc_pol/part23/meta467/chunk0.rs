//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1368/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1368<F: Float>(t2904: F, t77139: F, t951: F, t959: F, t21091: F, t4483: F, t17564: F, t60722: F, t21589: F, t77119: F, t77122: F, t77124: F, t77127: F, t77130: F, t77133: F, t77135: F, t77138: F) -> (F, F, F, F, F) {
    let t77143 = F::cast_from(0.35089341735807877242e1_f64) * t959 * t2904 * t77139 * t951;
    let t77145 = F::cast_from(0.14035736694323150897e2_f64) * t4483 * t21091;
    let t77148 = F::cast_from(0.61524113149298439947e4_f64) * t959 * t17564 * t60722;
    let t77150 = F::cast_from(0.23392894490538584828e1_f64) * t4483 * t21589;
    let t77151 = t77119 - t77122 - t77124 + t77127 - t77130 - t77133 + t77135 + t77138 + t77143 - t77145 - t77148 - t77150;
    (t77143, t77145, t77148, t77150, t77151)
}
