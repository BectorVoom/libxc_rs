//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 792/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk792(t360: f64, t5866: f64, t68: f64, t6744: f64, t1935: f64, t23419: f64, t23469: f64, t23510: f64, t25639: f64, t25642: f64, t25683: f64, t28558: f64, t28566: f64, t28572: f64, t28578: f64, t28582: f64, t378: f64, t5885: f64, t5890: f64, t5894: f64, t5900: f64, t5909: f64, t6717: f64, t6742: f64, t6765: f64, t7574: f64, t7578: f64, t7583: f64) -> f64 {
    let t28586 = t5866 * t68 * t360;
    let t28587 = t6744 * t28586;
    let t28592 = -t23469 - t6765 * t5900 / 1152.0_f64 - t6717 * t5885 / 144.0_f64 - 0.20186378047070195428e-3_f64 * t7574 * t7578 - 0.10093189023535097714e-3_f64 * t1935 * t28558 + t6717 * t5890 / 288.0_f64 + t6717 * t5894 / 216.0_f64 - 0.10093189023535097714e-3_f64 * t1935 * t28566 + t23419 * t5909 / 1152.0_f64 - 0.20186378047070195428e-3_f64 * t25639 + t28572 * t378 / 1536.0_f64 + 0.20186378047070195428e-3_f64 * t25642 + 0.20186378047070195428e-3_f64 * t23510 * t28578 - 0.10093189023535097714e-3_f64 * t23510 * t28582 + 0.10093189023535097714e-3_f64 * t6742 * t28587 + 0.20186378047070195428e-3_f64 * t25683 * t7583;
    t28592
}
