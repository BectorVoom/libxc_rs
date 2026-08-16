//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 693/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk693(t7595: f64, t7628: f64, t7663: f64, t8739: f64, t9458: f64, t9460: f64, t9904: f64, t9906: f64, t9909: f64, t9911: f64, t9913: f64, t9915: f64, t9917: f64, t9919: f64, t9921: f64, t9923: f64) -> f64 {
    let t9925 = -t9458 - 0.79828278012425390428e-1_f64 * t8739 + t9460 + t7595 - 0.2727466165424534173e-1_f64 * t9904 - 0.12700854093841289481e-2_f64 * t9906 - t7628 - 0.99785347515531738034e-2_f64 * t9909 + 0.39828462315181744016e-3_f64 * t9911 - 0.33190385262651453347e-3_f64 * t9913 - 0.99785347515531738034e-2_f64 * t9915 + 0.14967802127329760705e-1_f64 * t9917 - 0.27879923620627220811e-2_f64 * t9919 - 0.5987120850931904282e-1_f64 * t9921 - 0.13276154105060581339e-2_f64 * t9923 - t7663;
    t9925
}
