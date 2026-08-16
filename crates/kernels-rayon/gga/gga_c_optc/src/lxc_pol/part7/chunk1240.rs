//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1240/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1240(t10977: f64, t11374: f64, t11526: f64, t23825: f64, t23951: f64, t25440: f64, t25468: f64, t25479: f64, t25547: f64, t25552: f64, t25561: f64, t25562: f64, t25566: f64, t25570: f64, t25591: f64, t25595: f64, t2601: f64, t2672: f64, t2704: f64, t2721: f64, t2722: f64, t2812: f64, t3813: f64, t3836: f64, t3884: f64, t3917: f64, t7380: f64, t7865: f64, t7872: f64, t8049: f64, t8052: f64, t8140: f64, t8201: f64, t894: f64, t914: f64, t930: f64, t953: f64) -> f64 {
    let t25604 = 0.13613985915860191978e1_f64 * t25547 + 0.10747883617784362088e1_f64 * t2704 * t7872 - 0.30909018630360027928e0_f64 * t25552 + 0.49903344976940985984e3_f64 * t8140 * t8052 + 0.18137053605011111024e0_f64 * t953 * t894 * t7865 * t23825 + 0.45352564237957702055e6_f64 * t25561 * t25562 * t7380 * t25566 - 0.45352564237957702055e6_f64 * t25570 * t25562 * t2672 * t25566 + 0.35163949364965747848e4_f64 * t11526 * t25440 * t2672 * t10977 + 0.389869882632351453e2_f64 * t2812 * t3836 * t25468 + 0.64487301706706172529e0_f64 * t2704 * t8049 + 0.90880810212048753088e1_f64 * t2721 * t2722 * t25479 - 0.23181763972770020945e0_f64 * t930 * t914 * t2601 * t23951 - 0.45440405106024376544e1_f64 * t2721 * t2722 * t25591 - 0.35163949364965747848e4_f64 * t3917 * t11374 * t25595 * t8201 + 0.17581974682482873924e4_f64 * t3884 * t11374 * t25595 * t3813;
    t25604
}
