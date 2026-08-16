//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2921/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2921<F: Float>(t17948: F, t2940: F, t17564: F, t2933: F, t959: F, t17934: F, t2952: F, t1589: F, t48766: F, t14473: F, t4493: F, t18169: F, t3216: F) -> (F, F, F, F, F, F) {
    let t60857 = F::cast_from(0.20779030926817756511e3_f64) * t2940 * t17948;
    let t60860 = F::cast_from(0.6233709278045326953e3_f64) * t959 * t17564 * t2933;
    let t60862 = F::cast_from(0.17315859105681463759e2_f64) * t17934 * t2952;
    let t60864 = F::cast_from(0.11696447245269292414e1_f64) * t48766 * t1589;
    let t60866 = F::cast_from(0.23392894490538584828e1_f64) * t14473 * t4493;
    let t60867 = t18169 * t3216;
    (t60857, t60860, t60862, t60864, t60866, t60867)
}
