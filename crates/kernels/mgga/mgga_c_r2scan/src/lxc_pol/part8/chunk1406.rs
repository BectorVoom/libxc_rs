//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1406/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1406<F: Float>(t133: F, t1604: F, t1605: F, t9880: F, t2731: F, t9469: F, t2727: F, t9464: F, t22856: F, t9978: F, t20858: F, t20869: F, t30047: F, t30051: F, t30055: F, t30059: F, t30069: F, t30072: F, t30092: F) -> (F,) {
    let t34095 = t1604 * t1605 * t133 * t9880;
    let t34102 = t9469 * t2731;
    let t34104 = t9464 * t2727;
    let t34106 = t22856 * t9978;
    let t34112 = 0.54878743191129263322e-2 * t34095 + 0.2037639021386884617e0 * t30047 - 0.14636160809074174528e-1 * t30051 - 0.1047928639570397803e0 * t30055 - 0.1047928639570397803e0 * t30059 - 0.69345773920434148506e0 * t30069 + 0.10401866088065122276e1 * t34102 + 0.34672886960217074253e0 * t34104 + 0.15366048093516193731e1 * t34106 - 0.58544643236296698112e-1 * t30072 + 0.5141876673348786705e0 * t20858 + 0.18496169001454677638e1 * t20869 - 0.29272321618148349056e-1 * t30092;
    (t34112,)
}
