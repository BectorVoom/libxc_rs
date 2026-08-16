//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1194/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1194(t1079: f64, t1695: f64, t6244: f64, t11133: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64) -> (f64, f64) {
    let t23583 = t1079 * t6244 * t1695;
    let t23598 = -t11133 - 0.19755555555555555556e-1_f64 * t15189 + 0.9877777777777777778e-2_f64 * t18919 - 0.29633333333333333334e-1_f64 * t18924 + 0.14816666666666666667e-1_f64 * t18934 - 0.16462962962962962963e-1_f64 * t23479 + 0.59266666666666666668e-1_f64 * t23483 - 0.29633333333333333334e-1_f64 * t23501 - 0.88900000000000000002e-1_f64 * t23487 + 0.88900000000000000002e-1_f64 * t23505 - 0.14816666666666666667e-1_f64 * t23490;
    (t23583, t23598)
}
