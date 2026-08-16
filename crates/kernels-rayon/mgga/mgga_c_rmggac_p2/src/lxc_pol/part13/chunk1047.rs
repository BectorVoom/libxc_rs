//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1047/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1047(t39031: f64, t34903: f64, t34905: f64, t34907: f64, t34911: f64, t34913: f64, t39033: f64, t39036: f64, t39039: f64, t39042: f64, t39046: f64, t39048: f64, t39057: f64, t39061: f64, t39065: f64, t39068: f64, t39073: f64, t39079: f64) -> f64 {
    let t42823 = 0.10909864661698136692e0_f64 * t39031;
    let t42841 = -t42823 - 0.68186654135613354325e-2_f64 * t39033 + 0.16364796992547205038e0_f64 * t39036 + 0.81823984962736025191e-1_f64 * t39039 + 0.5987120850931904282e-1_f64 * t39042 + 0.40911992481368012596e-1_f64 * t39046 + 0.14546486215597515589e0_f64 * t39048 + 0.49658699875514145964e-4_f64 * t34903 + 0.24829349937757072982e-4_f64 * t34905 + 0.39726959900411316772e-4_f64 * t34907 + 0.59590439850616975158e-4_f64 * t34911 - 0.59590439850616975158e-4_f64 * t34913 + 0.81823984962736025192e-1_f64 * t39057 - 0.16364796992547205038e0_f64 * t39061 - 0.40911992481368012596e-1_f64 * t39065 + 0.81823984962736025192e-1_f64 * t39068 - 0.47885174879960069324e-4_f64 * t39073 + 0.14365552463988020797e-3_f64 * t39079;
    t42841
}
