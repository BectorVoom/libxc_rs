//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1086/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1086<F: Float>(t38147: F, t38150: F, t38153: F, t38156: F, t38158: F, t38161: F, t38649: F, t38657: F, t41756: F, t41757: F, t41762: F, t41763: F, t40251: F, t40257: F, t40260: F, t38166: F, t38170: F, t38177: F, t38183: F, t38185: F, t38191: F, t38193: F, t38661: F, t38666: F) -> (F, F) {
    let t41766 = t41756 + t41757 - t38649 + 0.93149212406257582492e-1 * t38147 + 0.32524801797942610063e-3 * t38150 - 0.11565819519348392138e-2 * t38153 + 0.27944763721877274748e0 * t38156 - t41762 - t41763 + 0.12805040077930161442e0 * t38158 - 0.93149212406257582492e-1 * t38161 + t38657;
    let t41770 = 0.35707476898255463229e0 * t40251;
    let t41775 = 0.21951497276451705328e-1 * t40257;
    let t41776 = 0.27944763721877274748e0 * t40260;
    let t41777 = 0.16951189180550569635e1 * t38166 + 0.90044238659382329742e0 * t38170 + t38661 - 0.13170898365871023197e0 * t38177 - t41770 - 0.65854491829355115986e-1 * t38183 + 0.58544643236296698111e-1 * t38185 + t38666 - 0.10975748638225852664e-1 * t38191 + 0.10975748638225852664e-1 * t38193 + t41775 - t41776;
    (t41766, t41777)
}
