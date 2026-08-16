//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 625/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk625(t118: f64, t8244: f64, t5148: f64, t5266: f64, t7783: f64, t7786: f64, t7789: f64, t7793: f64, t7795: f64, t7797: f64, t7811: f64, t7813: f64, t7815: f64, t7826: f64, t7830: f64, t7832: f64, t7838: f64, t7842: f64, t8232: f64, t8236: f64, t8242: f64, t8243: f64) -> f64 {
    let t8245 = t118 * t8244;
    let t8252 = -0.5454932330849068346e-1_f64 * t7783 + 0.16364796992547205038e0_f64 * t7786 + 0.40911992481368012596e-1_f64 * t7789 + 0.15965655602485078085e0_f64 * t7793 + 0.47896966807455234256e0_f64 * t7795 + 0.5987120850931904282e-1_f64 * t7797 + 0.23948483403727617128e0_f64 * t5266 * t8232 - 0.23948483403727617128e0_f64 * t5148 * t8236 + 0.5987120850931904282e-1_f64 * t7811 - 0.8980681276397856423e-1_f64 * t7813 - 0.3193131120497015617e0_f64 * t7815 + t8242 - t8243 + 0.19957069503106347607e-1_f64 * t8245 + 0.10909864661698136692e0_f64 * t7826 - 0.13637330827122670865e0_f64 * t7830 - 0.1454648621559751559e0_f64 * t7832 - 0.13637330827122670865e-1_f64 * t7838 - 0.36366215538993788974e-1_f64 * t7842;
    t8252
}
