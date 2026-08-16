//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1293/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1293(t5: f64, t125855: f64, t125900: f64, t112: f64, t671: f64, t8859: f64, t117773: f64, t120125: f64, t120127: f64, t120129: f64, t120131: f64, t120137: f64, t120140: f64, t120165: f64, t123084: f64, t123086: f64, t125100: f64, t1458: f64, t32609: f64, t4072: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t125902 = piecewise3(t8, 0.0_f64, t125855 + t125900);
    let t125903 = t125902 * t112;
    let t125910 = t8859 * t671;
    let t125915 = 2.0_f64 * t117773 * t1458 + 2.0_f64 * t125100 * t671 + 2.0_f64 * t125910 * t1458 + 2.0_f64 * t32609 * t4072 + t120125 + t120127 + t120129 + t120131 + t120137 + t120140 + t120165 + 4.0_f64 * t123084 + 4.0_f64 * t123086 + t125903;
    (t125903, t125910, t125915)
}
