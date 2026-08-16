//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 775/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk775(t4979: f64, t8912: f64, t3113: f64, t3707: f64, t3112: f64, t3117: f64, t3123: f64, t8798: f64, t611: f64, t8769: f64, t5409: f64, t8878: f64, t8881: f64, t8886: f64, t8889: f64, t8891: f64, t8898: f64, t8901: f64, t8904: f64, t8908: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8913 = t8912 * t4979;
    let t8915 = t3113 * t3707;
    let t8916 = t3112 * t8915;
    let t8917 = t8916 * t3117;
    let t8919 = t8798 * t3123;
    let t8921 = t611 * t8769;
    let t8922 = t8921 * t5409;
    let t8924 = -0.84412963981222021454e-7_f64 * t8878 - 0.10005749997240850277e-7_f64 * t8881 - 0.49240895655712845848e-7_f64 * t8886 + 0.12380568050579229813e-5_f64 * t8889 - 0.12163329537032409896e-2_f64 * t8891 - 0.37545833188964626383e-6_f64 * t8898 + 0.2318836277704281739e-4_f64 * t8901 + 0.20241536458333333334e-4_f64 * t8904 - 0.89947526170248609072e-8_f64 * t8908 + 0.1199450261480265202e-7_f64 * t8913 - 0.45020247456651744774e-7_f64 * t8917 - 0.44082325634638166759e-6_f64 * t8919 + 0.25745714186718600948e-5_f64 * t8922;
    (t8913, t8915, t8916, t8917, t8919, t8922, t8924)
}
