//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1234/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1234(t40251: f64, t40257: f64, t40260: f64, t38166: f64, t38170: f64, t38177: f64, t38183: f64, t38185: f64, t38191: f64, t38193: f64, t38661: f64, t38666: f64) -> f64 {
    let t41770 = 0.35707476898255463229e0_f64 * t40251;
    let t41775 = 0.21951497276451705328e-1_f64 * t40257;
    let t41776 = 0.27944763721877274748e0_f64 * t40260;
    let t41777 = 0.16951189180550569635e1_f64 * t38166 + 0.90044238659382329742e0_f64 * t38170 + t38661 - 0.13170898365871023197e0_f64 * t38177 - t41770 - 0.65854491829355115986e-1_f64 * t38183 + 0.58544643236296698111e-1_f64 * t38185 + t38666 - 0.10975748638225852664e-1_f64 * t38191 + 0.10975748638225852664e-1_f64 * t38193 + t41775 - t41776;
    t41777
}
