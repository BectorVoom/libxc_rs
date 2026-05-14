//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 590/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk590<F: Float>(t7820: F, t338: F, t8159: F, t118: F, t5148: F, t5266: F, t7783: F, t7786: F, t7789: F, t7793: F, t7795: F, t7797: F, t7811: F, t7813: F, t7815: F, t7826: F, t7830: F, t7832: F, t7838: F, t7842: F, t8232: F, t8236: F, t8242: F) -> (F, F, F) {
    let t8243 = 0.66671395154821946452e-1 * t7820;
    let t8244 = t338 * t8159;
    let t8245 = t118 * t8244;
    let t8252 = -0.5454932330849068346e-1 * t7783 + 0.16364796992547205038e0 * t7786 + 0.40911992481368012596e-1 * t7789 + 0.15965655602485078085e0 * t7793 + 0.47896966807455234256e0 * t7795 + 0.5987120850931904282e-1 * t7797 + 0.23948483403727617128e0 * t5266 * t8232 - 0.23948483403727617128e0 * t5148 * t8236 + 0.5987120850931904282e-1 * t7811 - 0.8980681276397856423e-1 * t7813 - 0.3193131120497015617e0 * t7815 + t8242 - t8243 + 0.19957069503106347607e-1 * t8245 + 0.10909864661698136692e0 * t7826 - 0.13637330827122670865e0 * t7830 - 0.1454648621559751559e0 * t7832 - 0.13637330827122670865e-1 * t7838 - 0.36366215538993788974e-1 * t7842;
    (t8243, t8244, t8252)
}
