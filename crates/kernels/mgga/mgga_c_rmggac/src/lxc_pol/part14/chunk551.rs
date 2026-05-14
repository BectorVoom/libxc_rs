//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 551/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk551<F: Float>(t7600: F, t7603: F, t7586: F, t793: F, t7590: F, t797: F, t851: F, t854: F, t36: F, t839: F, t3814: F, t265: F, t333: F, t7596: F, t3810: F, t7583: F, t7584: F, t7588: F, t7592: F, t7595: F, t7597: F, t7601: F) -> (F, F, F, F, F) {
    let t7604 = t7603 * t7600;
    let t7606 = t793 * t7586;
    let t7608 = t797 * t7590;
    let t7610 = t851 * t7586;
    let t7612 = t854 * t7590;
    let t7614 = t36 * t839;
    let t7615 = t3814 * t7614;
    let t7617 = t265 * t333;
    let t7618 = t797 * t7617;
    let t7620 = t851 * t7596;
    let t7622 = t3810 * t7614;
    let t7624 = t7583 + 0.39828462315181744016e-2 * t7584 + 0.9072038638458063915e-4 * t7588 - 0.10584045078201074568e-3 * t7592 + t7595 + 0.53218852008283593619e-1 * t7597 - 0.2727466165424534173e-1 * t7601 - 0.12700854093841289481e-2 * t7604 - 0.99785347515531738034e-2 * t7606 + 0.14967802127329760705e-1 * t7608 - 0.33190385262651453347e-3 * t7610 + 0.39828462315181744016e-3 * t7612 - 0.5987120850931904282e-1 * t7615 - 0.79828278012425390428e-1 * t7618 + 0.17701538806747441785e-2 * t7620 - 0.27879923620627220811e-2 * t7622;
    (t7614, t7617, t7618, t7620, t7624)
}
