//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 639/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk639<F: Float>(t36: F, t9908: F, t854: F, t9876: F, t851: F, t9872: F, t793: F, t797: F, t3810: F, t9888: F, t3814: F, t3839: F, t9884: F, t7595: F, t7628: F, t7663: F, t8739: F, t9458: F, t9460: F, t9904: F, t9906: F) -> (F,) {
    let t9909 = t9908 * t36;
    let t9911 = t854 * t9876;
    let t9913 = t851 * t9872;
    let t9915 = t793 * t9872;
    let t9917 = t797 * t9876;
    let t9919 = t3810 * t9888;
    let t9921 = t3814 * t9888;
    let t9923 = t3839 * t9884;
    let t9925 = -t9458 - 0.79828278012425390428e-1 * t8739 + t9460 + t7595 - 0.2727466165424534173e-1 * t9904 - 0.12700854093841289481e-2 * t9906 - t7628 - 0.99785347515531738034e-2 * t9909 + 0.39828462315181744016e-3 * t9911 - 0.33190385262651453347e-3 * t9913 - 0.99785347515531738034e-2 * t9915 + 0.14967802127329760705e-1 * t9917 - 0.27879923620627220811e-2 * t9919 - 0.5987120850931904282e-1 * t9921 - 0.13276154105060581339e-2 * t9923 - t7663;
    (t9925,)
}
