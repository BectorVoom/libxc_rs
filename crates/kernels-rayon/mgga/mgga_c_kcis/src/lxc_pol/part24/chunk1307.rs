//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1307/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1307(t101047: f64, t101231: f64, t101524: f64, t26685: f64, t26692: f64, t26748: f64, t27772: f64, t27773: f64, t28984: f64, t28988: f64, t4972: f64, t7703: f64, t93759: f64, t93762: f64, t96456: f64, t96482: f64, t96504: f64) -> f64 {
    let t101588 = -t96456 - 0.15445601851851851852e-3_f64 * t93759 - 0.15445601851851851852e-3_f64 * t93762 + 0.41703125000000000001e-2_f64 * t7703 * t101047 - 0.12356481481481481482e-2_f64 * t26692 * t28984 - t96482 - 0.27802083333333333334e-2_f64 * t7703 * t27772 * t27773 * t4972 + 0.18550940104166666667e-3_f64 * t26685 * t101231 - 0.13901041666666666667e-2_f64 * t26748 * t28988 - 0.13901041666666666667e-2_f64 * t7703 * t101524 - 0.61836467013888888889e-4_f64 * t96504;
    t101588
}
