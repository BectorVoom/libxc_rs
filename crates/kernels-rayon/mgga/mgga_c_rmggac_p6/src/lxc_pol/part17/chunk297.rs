//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 297/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk297(t1704: f64, t1707: f64, t1734: f64, t1737: f64, t1743: f64, t305: f64, t326: f64, t344: f64, t349: f64, t793: f64, t797: f64, t838: f64, t851: f64, t854: f64, t861: f64) -> f64 {
    let t1756 = 0.39914139006212695214e-1_f64 * t793 * t1704 - 0.11974241701863808564e0_f64 * t797 * t1707 + 0.19957069503106347607e-1_f64 * t305 * t1734 + 0.79828278012425390428e-1_f64 * t838 * t1737 - 0.19957069503106347607e-1_f64 * t326 * t1743 + 0.13276154105060581339e-2_f64 * t851 * t1704 - 0.31862769852145395214e-2_f64 * t854 * t1707 + 0.26552308210121162678e-3_f64 * t344 * t1734 + 0.18586615747084813875e-2_f64 * t861 * t1737 - 0.26552308210121162678e-3_f64 * t349 * t1743;
    t1756
}
