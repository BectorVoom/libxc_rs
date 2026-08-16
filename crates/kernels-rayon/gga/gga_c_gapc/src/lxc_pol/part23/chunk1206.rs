//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1206/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1206(t11597: f64, t3008: f64, t3060: f64, t1030: f64, t11591: f64, t144: f64, t1461: f64, t8709: f64, t11601: f64, t9288: f64, t34905: f64, t34907: f64, t34909: f64, t34911: f64, t34914: f64, t34918: f64, t34921: f64, t34926: f64) -> f64 {
    let t34929 = t3060 * t11597 * t3008;
    let t34934 = t1030 * t1461 * t8709 * t144 * t11591;
    let t34936 = t11601 * t9288;
    let t34938 = -0.24583187891642252608e-8_f64 * t34905 + 0.32777583855523003478e-8_f64 * t34907 - 0.8433973524305555556e-6_f64 * t34909 + 0.73797268337673611116e-6_f64 * t34911 + 0.73797268337673611116e-6_f64 * t34914 + 0.4423264264475966605e-6_f64 * t34918 + 0.22467583330805503619e-6_f64 * t34921 - 0.11666996708622685185e-3_f64 * t34926 + 0.13506074236995523433e-5_f64 * t34929 - 0.10957550886745307093e-6_f64 * t34934 + 0.67530371184977617164e-6_f64 * t34936;
    t34938
}
