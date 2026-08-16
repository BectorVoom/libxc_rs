//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1074/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1074<F: Float>(t75166: F, t75169: F, t75174: F, t75180: F, t75184: F, t77484: F, t77485: F, t77486: F, t77487: F, t77491: F, t77502: F, t77503: F, t77504: F, t77505: F, t77506: F, t77507: F, t77508: F) -> F {
    let t80231 = -t77484 + t77485 + t77486 + t77487 - t77491 + F::cast_from(0.10511583655740820312e-5_f64) * t75166 - F::cast_from(0.52557918278704101558e-5_f64) * t75169 + F::cast_from(0.29085809927086856922e-4_f64) * t75174 + F::cast_from(0.72714524817717142305e-5_f64) * t75180 - F::cast_from(0.72714524817717142305e-5_f64) * t75184 + t77502 + t77503 + t77504 - t77505 - t77506 + t77507 + t77508;
    t80231
}
