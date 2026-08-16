//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 767/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk767(t1564: f64, t15917: f64, t446: f64, t15752: f64, t447: f64, t15756: f64, t3281: f64, t432: f64, t4462: f64, t15604: f64, t15606: f64, t15609: f64, t15612: f64, t15617: f64, t15621: f64, t15628: f64, t15888: f64, t15891: f64, t15894: f64, t15897: f64, t15899: f64, t15904: f64, t15907: f64, t15910: f64, t15915: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15918 = t1564 * t15917;
    let t15919 = t446 * t15918;
    let t15921 = t447 * t15752;
    let t15922 = t446 * t15921;
    let t15924 = t447 * t15756;
    let t15925 = t3281 * t15924;
    let t15927 = t4462 * t432;
    let t15928 = t1564 * t15927;
    let t15929 = t446 * t15928;
    let t15931 = -t15604 + t15606 / 81.0_f64 - t15609 / 27.0_f64 + t15612 / 54.0_f64 + t15617 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t15621 - t15628 / 18.0_f64 - t15888 / 6.0_f64 + t15891 / 18.0_f64 - t15894 / 9.0_f64 - 4.0_f64 / 27.0_f64 * t15897 - t15899 / 27.0_f64 + t15904 / 18.0_f64 + t15907 / 9.0_f64 - t15910 / 27.0_f64 - t15915 / 9.0_f64 - t15919 / 9.0_f64 - t15922 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t15925 + t15929 / 18.0_f64;
    (t15919, t15922, t15925, t15927, t15929, t15931)
}
