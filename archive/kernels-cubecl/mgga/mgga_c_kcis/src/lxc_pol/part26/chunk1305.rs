//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1305/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1305<F: Float>(t21935: F, t4153: F, t7923: F, t21939: F, t101875: F, t101950: F, t102005: F, t102068: F, t102311: F, t102313: F, t102318: F, t102328: F, t102334: F, t27595: F, t7968: F, t7981: F) -> (F, F, F) {
    let t102337 = t4153 * t7923 * t21935;
    let t102340 = t4153 * t7923 * t21939;
    let t102342 = F::cast_from(0.77382407407407407407e-3_f64) * t102311 - F::cast_from(0.15476481481481481481e-2_f64) * t102313 - F::cast_from(0.11326774691358024691e-2_f64) * t101950 * t7981 - F::cast_from(0.51485339506172839507e-4_f64) * t102318 - F::cast_from(0.46377350260416666667e-4_f64) * t7968 * t102068 - F::cast_from(0.13913205078125e-3_f64) * t7968 * t102005 - F::cast_from(0.92835860883789062501e-5_f64) * t27595 * t102005 + F::cast_from(0.92835860883789062501e-5_f64) * t27595 * t102328 + F::cast_from(0.557015165302734375e-4_f64) * t27595 * t101875 - F::cast_from(0.19345601851851851852e-2_f64) * t102334 + F::cast_from(0.12897067901234567901e-2_f64) * t102337 - F::cast_from(0.11607361111111111111e-1_f64) * t102340;
    (t102337, t102340, t102342)
}
