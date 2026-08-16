//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 543/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk543(t4551: f64, t4564: f64, t1611: f64, t1620: f64, t240: f64, t4164: f64, t4167: f64, t4173: f64, t4322: f64, t4528: f64, t4530: f64, t4535: f64, t4536: f64, t555: f64) -> (f64, f64) {
    let t4565 = t4551 + t4564;
    let t4569 = t4164 - t4167 + t4173 - t4322 + t240 * (-t1611 * t4565 - 2.0_f64 * t1620 * t4530 + t4528 * t555 + 2.0_f64 * t4535 * t4536 - t4164 + t4167 - t4173 + t4322);
    (t4565, t4569)
}
