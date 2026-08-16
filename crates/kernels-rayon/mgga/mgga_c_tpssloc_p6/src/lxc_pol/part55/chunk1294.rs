//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1294/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1294(t120044: f64, t120063: f64, t120067: f64, t120069: f64, t120075: f64, t120078: f64, t120083: f64, t120085: f64, t123155: f64, t123164: f64, t123168: f64, t123173: f64, t125818: f64, t125915: f64, t1849: f64, t2114: f64, t27858: f64, t31055: f64, t31057: f64, t31060: f64, t32623: f64, t574: f64) -> f64 {
    let t125919 = t120044 - 4.0_f64 * t123155 - t31055 - t31057 - t31060 - 4.0_f64 * t123164 - t120063 - t120067 - t120069 - t120075 - 2.0_f64 * t2114 * t27858 + t120078 + t32623 * t1849 - 4.0_f64 * t123168 - t120083 + (t125818 + t125915) * t574 + t120085 + 6.0_f64 * t123173;
    t125919
}
