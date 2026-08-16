//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1087/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1087(t14991: f64, t2740: f64, t14473: f64, t3923: f64, t11508: f64, t11524: f64, t11528: f64, t11550: f64, t11562: f64, t14956: f64, t14960: f64, t14965: f64, t14970: f64, t14975: f64, t14980: f64, t14987: f64, t2682: f64, t2685: f64, t4966: f64, t4970: f64, t4974: f64, t4985: f64, t4991: f64, t8509: f64, t8954: f64, t8989: f64, t925: f64) -> f64 {
    let t14992 = t2740 * t14991;
    let t14994 = t3923 * t14473;
    let t14997 = -t8954 / 20736.0_f64 + t925 * t14956 / 288.0_f64 - t14960 / 432.0_f64 - t2685 * t4974 / 108.0_f64 + t14965 / 864.0_f64 - t8989 * t4985 / 432.0_f64 + 5.0_f64 / 6912.0_f64 * t2740 * t14970 - t8509 * t14975 / 2304.0_f64 + t11508 + t14980 / 4608.0_f64 - t2682 * t4991 / 576.0_f64 - t11524 + t11528 + t11550 - t2685 * t4966 / 81.0_f64 + t14987 / 648.0_f64 + t2685 * t4970 / 54.0_f64 - t11562 + t14992 / 3456.0_f64 + t925 * t14994 / 48.0_f64;
    t14997
}
