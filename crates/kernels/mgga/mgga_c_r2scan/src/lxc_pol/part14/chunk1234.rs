//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1234/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1234<F: Float>(t40251: F, t40257: F, t40260: F, t38166: F, t38170: F, t38177: F, t38183: F, t38185: F, t38191: F, t38193: F, t38661: F, t38666: F) -> F {
    let t41770 = F::new(0.35707476898255463229e0) * t40251;
    let t41775 = F::new(0.21951497276451705328e-1) * t40257;
    let t41776 = F::new(0.27944763721877274748e0) * t40260;
    let t41777 = F::new(0.16951189180550569635e1) * t38166 + F::new(0.90044238659382329742e0) * t38170 + t38661 - F::new(0.13170898365871023197e0) * t38177 - t41770 - F::new(0.65854491829355115986e-1) * t38183 + F::new(0.58544643236296698111e-1) * t38185 + t38666 - F::new(0.10975748638225852664e-1) * t38191 + F::new(0.10975748638225852664e-1) * t38193 + t41775 - t41776;
    t41777
}
