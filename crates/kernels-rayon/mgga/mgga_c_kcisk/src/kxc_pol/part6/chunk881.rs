//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 881/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk881(t28578: f64, t28698: f64, t673: f64, t716: f64, t720: f64, t415: f64, t2533: f64, t8666: f64, t6719: f64, t8874: f64, t1869: f64, t23033: f64, t2527: f64) -> (f64, f64, f64, f64, f64) {
    let t28699 = t28578 + t28698;
    let t28700 = t673 * t28699;
    let t28701 = t28700 * t716;
    let t28702 = t28701 * t720;
    let t28703 = t415 * t28702;
    let t28705 = t8666 * t2533;
    let t28706 = t415 * t28705;
    let t28710 = t6719 * t8874;
    let t28711 = t1869 * t28710;
    let t28713 = t23033 * t2527;
    (t28699, t28703, t28706, t28711, t28713)
}
