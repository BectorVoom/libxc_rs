//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 529/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk529(t2386: f64, t45: f64, t2394: f64, t4761: f64, t2063: f64, t696: f64, t5136: f64, t2494: f64, t960: f64, t2497: f64, t965: f64, t2502: f64, t970: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6851 = t45 * t2386;
    let t6856 = t4761 * t2394;
    let t6903 = t696 * t2063;
    let t6906 = t5136 * t2063;
    let t6922 = t960 * t2494;
    let t6924 = t965 * t2497;
    let t6926 = t970 * t2502;
    (t6851, t6856, t6903, t6906, t6922, t6924, t6926)
}
