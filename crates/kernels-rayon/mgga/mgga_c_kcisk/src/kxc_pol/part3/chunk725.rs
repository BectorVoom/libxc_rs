//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 725/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk725(t3521: f64, t4611: f64, t1814: f64, t1876: f64, t1636: f64, t4658: f64, t1824: f64, t4644: f64, t4609: f64, t1646: f64, t1797: f64, t708: f64) -> (f64, f64, f64, f64) {
    let t11257 = t3521 * t4611;
    let t11259 = t1876 * t1814;
    let t11260 = t1636 * t4658;
    let t11261 = t11259 * t11260;
    let t11264 = t4644 * t1824;
    let t11265 = t4609 * t11264;
    let t11269 = t1797 * t1646 * t708;
    (t11257, t11261, t11265, t11269)
}
