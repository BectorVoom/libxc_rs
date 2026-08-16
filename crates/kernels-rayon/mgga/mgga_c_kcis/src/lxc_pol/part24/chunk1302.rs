//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1302/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1302(t27803: f64, t27832: f64, t1003: f64, t18476: f64, t26686: f64, t100680: f64, t100683: f64, t101084: f64, t26685: f64, t27808: f64, t27895: f64, t27911: f64, t27915: f64, t95524: f64, t96218: f64, t96231: f64) -> (f64, f64) {
    let t101464 = t27832 * t27803;
    let t101469 = t26686 * t18476 * t1003;
    let t101476 = -0.18550940104166666667e-3_f64 * t95524 * t27911 - 0.13901041666666666667e-2_f64 * t27832 * t27911 - 0.27802083333333333334e-2_f64 * t27832 * t27808 - 0.15445601851851851852e-3_f64 * t101464 + t96218 + 0.18550940104166666667e-3_f64 * t27895 * t27915 - t96231 - 0.92754700520833333333e-4_f64 * t26685 * t101469 - 0.2782641015625e-3_f64 * t26685 * t101084 + 0.22109259259259259259e-2_f64 * t100680 - 0.49745833333333333332e-2_f64 * t100683;
    (t101469, t101476)
}
