//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1078/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1078<F: Float>(t40623: F, t41146: F, t41160: F, t41132: F, t41134: F, t41136: F, t41138: F, t41140: F, t41142: F, t41144: F, t41148: F, t41151: F, t41153: F, t41155: F, t41158: F, t41162: F) -> (F, F) {
    let t43492 = F::new(0.2927036860455597649e0) * t40623;
    let t43507 = F::new(0.3193131120497015617e0) * t41146;
    let t43513 = F::new(0.14161231045397953428e-1) * t41160;
    let t43515 = F::new(0.3628815455383225566e-2) * t41132 + F::new(0.11974241701863808564e0) * t41134 + F::new(0.11974241701863808564e0) * t41136 + F::new(0.11974241701863808564e0) * t41138 + F::new(0.59871208509319042821e-1) * t41140 - F::new(0.26552308210121162678e-2) * t41142 - F::new(0.11974241701863808564e0) * t41144 - t43507 + F::new(0.5987120850931904282e0) * t41148 - F::new(0.10909864661698136692e0) * t41151 + F::new(0.2727466165424534173e0) * t41153 + F::new(0.44607877793003553299e-1) * t41155 - F::new(0.79656924630363488034e-2) * t41158 + t43513 - F::new(0.27879923620627220812e-1) * t41162;
    (t43492, t43515)
}
