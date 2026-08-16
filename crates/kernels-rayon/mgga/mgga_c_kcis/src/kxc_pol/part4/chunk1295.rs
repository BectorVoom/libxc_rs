//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1295/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1295(t16377: f64, t16454: f64, t16559: f64, t16607: f64, t1396: f64, t1468: f64, t1464: f64, t4137: f64, t5752: f64, t1928: f64, t4134: f64, t4136: f64) -> (f64, f64, f64, f64) {
    let t16609 = t16377 + t16454 + t16559 + t16607;
    let t16610 = t1396 * t16609;
    let t16611 = t1468 * t16610;
    let t16612 = t1464 * t16611;
    let t16614 = t5752 * t4137;
    let t16615 = t1464 * t16614;
    let t16617 = t1928 * t4134;
    let t16618 = t16617 * t4136;
    (t16609, t16612, t16615, t16618)
}
