//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 365/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk365(t1646: f64, t677: f64, t1634: f64, t1638: f64, t1649: f64, t1648: f64, t574: f64) -> (f64, f64, f64) {
    let t1815 = t1646 * t677;
    let t1819 = 0.41275e-2_f64 * t1634;
    let t1821 = 0.1982e-1_f64 * t1649 - t1819 - 0.41275e-2_f64 * t1638;
    let t1824 = t1815 * t1648 / 4.0_f64 + t574 * t1821 / 2.0_f64;
    (t1815, t1821, t1824)
}
