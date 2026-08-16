//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 681/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk681(t1814: f64, t1876: f64, t1646: f64, t1797: f64, t708: f64, t4594: f64, t574: f64, t4595: f64, t682: f64, t139: f64, t3516: f64, t41: f64) -> (f64, f64, f64, f64, f64) {
    let t11259 = t1876 * t1814;
    let t11269 = t1797 * t1646 * t708;
    let t11279 = t4594 * t574 * t708;
    let t11285 = t4595 * t682;
    let t11313 = t139 * t3516 * t41;
    (t11259, t11269, t11279, t11285, t11313)
}
