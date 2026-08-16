//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1240/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1240<F: Float>(t10977: F, t11374: F, t11526: F, t23825: F, t23951: F, t25440: F, t25468: F, t25479: F, t25547: F, t25552: F, t25561: F, t25562: F, t25566: F, t25570: F, t25591: F, t25595: F, t2601: F, t2672: F, t2704: F, t2721: F, t2722: F, t2812: F, t3813: F, t3836: F, t3884: F, t3917: F, t7380: F, t7865: F, t7872: F, t8049: F, t8052: F, t8140: F, t8201: F, t894: F, t914: F, t930: F, t953: F) -> F {
    let t25604 = F::cast_from(0.13613985915860191978e1_f64) * t25547 + F::cast_from(0.10747883617784362088e1_f64) * t2704 * t7872 - F::cast_from(0.30909018630360027928e0_f64) * t25552 + F::cast_from(0.49903344976940985984e3_f64) * t8140 * t8052 + F::cast_from(0.18137053605011111024e0_f64) * t953 * t894 * t7865 * t23825 + F::cast_from(0.45352564237957702055e6_f64) * t25561 * t25562 * t7380 * t25566 - F::cast_from(0.45352564237957702055e6_f64) * t25570 * t25562 * t2672 * t25566 + F::cast_from(0.35163949364965747848e4_f64) * t11526 * t25440 * t2672 * t10977 + F::cast_from(0.389869882632351453e2_f64) * t2812 * t3836 * t25468 + F::cast_from(0.64487301706706172529e0_f64) * t2704 * t8049 + F::cast_from(0.90880810212048753088e1_f64) * t2721 * t2722 * t25479 - F::cast_from(0.23181763972770020945e0_f64) * t930 * t914 * t2601 * t23951 - F::cast_from(0.45440405106024376544e1_f64) * t2721 * t2722 * t25591 - F::cast_from(0.35163949364965747848e4_f64) * t3917 * t11374 * t25595 * t8201 + F::cast_from(0.17581974682482873924e4_f64) * t3884 * t11374 * t25595 * t3813;
    t25604
}
