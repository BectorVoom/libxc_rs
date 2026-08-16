//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 407/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk407(t1684: f64, t1735: f64, t1732: f64, t1738: f64, t1762: f64, t1769: f64, t2061: f64, t2063: f64, t467: f64, t452: f64, t1748: f64, t2026: f64, t2027: f64, t2029: f64, t2030: f64, t2032: f64, t2037: f64, t2044: f64, t2047: f64, t2053: f64, t2058: f64, t2060: f64, t453: f64, t455: f64, t463: f64, t472: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2065 = 0.3056501876701794_f64 * t1684;
    let t2067 = 0.1018833958900598_f64 * t1735;
    let t2069 = t2061 - 1.5323028051206833_f64 * t1762 + t2063 + 1.5323028051206833_f64 * t1769 + t2065 - 0.3056501876701794_f64 * t1732 + t2067 + 0.3056501876701794_f64 * t1738;
    let t2070 = t467 * t2069;
    let t2071 = t2070 * t452;
    let t2074 = -t2026 - t2027 - 0.10237773105191754_f64 * t1738 - t2029 - t2030 + t463 * t2032 / 6.0_f64 - t2037 * t1748 / 6.0_f64 + t2044 - t2047 - t472 * t2032 / 6.0_f64 + t453 * t2032 / 6.0_f64 - t2053 * t1748 / 6.0_f64 + t2058 + t2060 + t2071 * t455 / 6.0_f64;
    (t2065, t2067, t2069, t2070, t2071, t2074)
}
