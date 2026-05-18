//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 667/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk667<F: Float>(t4838: F, t401: F, t4824: F, t1483: F, t1466: F, t1477: F, t402: F, t4741: F, t4744: F, t4746: F, t4748: F, t4751: F) -> (F, F, F, F, F, F, F, F) {
    let t4839 = F::new(1.0) * t4838;
    let t4840 = t4824 * t401;
    let t4841 = t1483 * t4840;
    let t4842 = F::new(6.0) * t4841;
    let t4844 = t1466 * t402 * t1477;
    let t4845 = F::new(6.0) * t4844;
    let t4849 = F::new(0.93932222222222222223e0) * t4741;
    let t4850 = F::new(0.73355e-1) * t4744;
    let t4851 = F::new(0.14671e0) * t4746;
    let t4852 = F::new(0.17116166666666666667e0) * t4748;
    let t4853 = F::new(0.36793333333333333333e0) * t4751;
    (t4839, t4842, t4845, t4849, t4850, t4851, t4852, t4853)
}
