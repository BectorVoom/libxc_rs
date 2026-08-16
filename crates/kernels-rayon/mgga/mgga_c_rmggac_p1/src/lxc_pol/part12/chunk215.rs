//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 215/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk215(t305: f64, t326: f64, t344: f64, t349: f64, t793: f64, t794: f64, t797: f64, t798: f64, t833: f64, t838: f64, t839: f64, t848: f64, t851: f64, t854: f64, t861: f64) -> f64 {
    let t866 = 0.39914139006212695214e-1_f64 * t793 * t794 - 0.11974241701863808564e0_f64 * t797 * t798 + 0.19957069503106347607e-1_f64 * t305 * t833 + 0.79828278012425390428e-1_f64 * t838 * t839 - 0.19957069503106347607e-1_f64 * t326 * t848 + 0.13276154105060581339e-2_f64 * t851 * t794 - 0.31862769852145395214e-2_f64 * t854 * t798 + 0.26552308210121162678e-3_f64 * t344 * t833 + 0.18586615747084813875e-2_f64 * t861 * t839 - 0.26552308210121162678e-3_f64 * t349 * t848;
    t866
}
