//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1078/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1078(t40623: f64, t41146: f64, t41160: f64, t41132: f64, t41134: f64, t41136: f64, t41138: f64, t41140: f64, t41142: f64, t41144: f64, t41148: f64, t41151: f64, t41153: f64, t41155: f64, t41158: f64, t41162: f64) -> (f64, f64) {
    let t43492 = 0.2927036860455597649e0_f64 * t40623;
    let t43507 = 0.3193131120497015617e0_f64 * t41146;
    let t43513 = 0.14161231045397953428e-1_f64 * t41160;
    let t43515 = 0.3628815455383225566e-2_f64 * t41132 + 0.11974241701863808564e0_f64 * t41134 + 0.11974241701863808564e0_f64 * t41136 + 0.11974241701863808564e0_f64 * t41138 + 0.59871208509319042821e-1_f64 * t41140 - 0.26552308210121162678e-2_f64 * t41142 - 0.11974241701863808564e0_f64 * t41144 - t43507 + 0.5987120850931904282e0_f64 * t41148 - 0.10909864661698136692e0_f64 * t41151 + 0.2727466165424534173e0_f64 * t41153 + 0.44607877793003553299e-1_f64 * t41155 - 0.79656924630363488034e-2_f64 * t41158 + t43513 - 0.27879923620627220812e-1_f64 * t41162;
    (t43492, t43515)
}
