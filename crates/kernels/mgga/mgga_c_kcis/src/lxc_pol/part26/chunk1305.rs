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
    let t102342 = F::new(0.77382407407407407407e-3) * t102311 - F::new(0.15476481481481481481e-2) * t102313 - F::new(0.11326774691358024691e-2) * t101950 * t7981 - F::new(0.51485339506172839507e-4) * t102318 - F::new(0.46377350260416666667e-4) * t7968 * t102068 - F::new(0.13913205078125e-3) * t7968 * t102005 - F::new(0.92835860883789062501e-5) * t27595 * t102005 + F::new(0.92835860883789062501e-5) * t27595 * t102328 + F::new(0.557015165302734375e-4) * t27595 * t101875 - F::new(0.19345601851851851852e-2) * t102334 + F::new(0.12897067901234567901e-2) * t102337 - F::new(0.11607361111111111111e-1) * t102340;
    (t102337, t102340, t102342)
}
