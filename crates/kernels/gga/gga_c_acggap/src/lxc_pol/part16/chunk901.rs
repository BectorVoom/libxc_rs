//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 901/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk901<F: Float>(t35458: F, t2450: F, t7646: F, t30468: F, t4741: F, t30216: F, t8526: F, t1983: F, t30692: F, t7586: F, t8901: F, t1992: F, t7585: F, t8906: F, t8402: F, t30105: F, t8897: F) -> (F, F, F, F, F, F, F, F) {
    let t35459 = 0.22642626448514989489e-1 * t35458;
    let t35466 = t2450 * t7646;
    let t35469 = t30468 * t4741;
    let t35471 = t30216 * t8526;
    let t35475 = t30692 * t7586 * t1983 * t8901;
    let t35476 = 0.7145669686344956162e-3 * t35475;
    let t35479 = t7585 * t7586 * t1992 * t8906;
    let t35480 = 0.28582678745379824648e-3 * t35479;
    let t35484 = t7585 * t7586 * t1983 * t8402;
    let t35485 = 0.14291339372689912324e-3 * t35484;
    let t35486 = t30105 * t8897;
    (t35459, t35466, t35469, t35471, t35476, t35480, t35485, t35486)
}
