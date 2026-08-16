//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 705/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk705(t1775: f64, t3515: f64, t1033: f64, t8282: f64, t3520: f64, t11717: f64, t3510: f64, t12306: f64, t12308: f64, t12310: f64, t12327: f64, t12356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12850 = 2.0_f64 / 9.0_f64 * t1775 * t3515;
    let t12852 = t8282 * t1033;
    let t12864 = 4.0_f64 / 3.0_f64 * t1775 * t3520;
    let t12865 = t11717 * t3510;
    let t12889 = 2.0_f64 / 27.0_f64 * t12306;
    let t12890 = 4.0_f64 / 27.0_f64 * t12308;
    let t12891 = 4.0_f64 / 81.0_f64 * t12310;
    let t12897 = 2.0_f64 / 27.0_f64 * t12327;
    let t12911 = 4.0_f64 / 9.0_f64 * t12356;
    (t12850, t12852, t12864, t12865, t12889, t12890, t12891, t12897, t12911)
}
