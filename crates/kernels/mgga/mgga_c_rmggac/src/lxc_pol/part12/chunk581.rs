//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 581/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk581<F: Float>(t7617: F, t797: F, t7596: F, t851: F, t3810: F, t7614: F, t7583: F, t7584: F, t7588: F, t7592: F, t7595: F, t7597: F, t7601: F, t7604: F, t7606: F, t7608: F, t7610: F, t7612: F, t7615: F) -> (F, F, F) {
    let t7618 = t797 * t7617;
    let t7620 = t851 * t7596;
    let t7622 = t3810 * t7614;
    let t7624 = t7583 + F::new(0.39828462315181744016e-2) * t7584 + F::new(0.9072038638458063915e-4) * t7588 - F::new(0.10584045078201074568e-3) * t7592 + t7595 + F::new(0.53218852008283593619e-1) * t7597 - F::new(0.2727466165424534173e-1) * t7601 - F::new(0.12700854093841289481e-2) * t7604 - F::new(0.99785347515531738034e-2) * t7606 + F::new(0.14967802127329760705e-1) * t7608 - F::new(0.33190385262651453347e-3) * t7610 + F::new(0.39828462315181744016e-3) * t7612 - F::new(0.5987120850931904282e-1) * t7615 - F::new(0.79828278012425390428e-1) * t7618 + F::new(0.17701538806747441785e-2) * t7620 - F::new(0.27879923620627220811e-2) * t7622;
    (t7618, t7620, t7624)
}
