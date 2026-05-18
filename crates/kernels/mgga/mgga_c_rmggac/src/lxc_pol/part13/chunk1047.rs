//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1047/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1047<F: Float>(t39031: F, t34903: F, t34905: F, t34907: F, t34911: F, t34913: F, t39033: F, t39036: F, t39039: F, t39042: F, t39046: F, t39048: F, t39057: F, t39061: F, t39065: F, t39068: F, t39073: F, t39079: F) -> F {
    let t42823 = F::new(0.10909864661698136692e0) * t39031;
    let t42841 = -t42823 - F::new(0.68186654135613354325e-2) * t39033 + F::new(0.16364796992547205038e0) * t39036 + F::new(0.81823984962736025191e-1) * t39039 + F::new(0.5987120850931904282e-1) * t39042 + F::new(0.40911992481368012596e-1) * t39046 + F::new(0.14546486215597515589e0) * t39048 + F::new(0.49658699875514145964e-4) * t34903 + F::new(0.24829349937757072982e-4) * t34905 + F::new(0.39726959900411316772e-4) * t34907 + F::new(0.59590439850616975158e-4) * t34911 - F::new(0.59590439850616975158e-4) * t34913 + F::new(0.81823984962736025192e-1) * t39057 - F::new(0.16364796992547205038e0) * t39061 - F::new(0.40911992481368012596e-1) * t39065 + F::new(0.81823984962736025192e-1) * t39068 - F::new(0.47885174879960069324e-4) * t39073 + F::new(0.14365552463988020797e-3) * t39079;
    t42841
}
