//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 712/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk712<F: Float>(t11051: F, t1714: F, t1707: F, t606: F, t11037: F, t1709: F, t4873: F, t4881: F, t4864: F, t10944: F, t10947: F, t10951: F, t10954: F, t10960: F, t10966: F, t11030: F, t11033: F, t11038: F) -> (F, F, F, F, F, F) {
    let t11052 = t1714 * t11051;
    let t11054 = t1707 * t11051;
    let t11056 = F::new(1.0)/pow_3_2::<f64>(t606);
    let t11057 = t11056 * t11037;
    let t11060 = t4881 * t1709 * t4873;
    let t11063 = t4864 * t1709 * t4873;
    let t11065 = -F::new(0.59793333333333333333e0) * t10944 + F::new(0.29896666666666666667e0) * t10947 - F::new(0.33218518518518518518e0) * t10951 + F::new(0.11958666666666666667e1) * t10954 - F::new(0.17938e1) * t10960 - F::new(0.29896666666666666667e0) * t10966 - t11030 - t11033 + F::new(0.142419375e1) * t11038 + F::new(0.3071625e0) * t11052 + F::new(0.1898925e1) * t11054 - F::new(0.76790625e-1) * t11057 + F::new(0.46074375e0) * t11060 - F::new(0.28483875e1) * t11063;
    (t11052, t11054, t11057, t11060, t11063, t11065)
}
