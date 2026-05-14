//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1127/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1127<F: Float>(t25560: F, t8207: F, t7373: F, t864: F, t769: F, t935: F, t1: F, t549: F, t3916: F, t2270: F, t7982: F, t1885: F, t2670: F, t10977: F, t11374: F, t11526: F, t23825: F, t23951: F, t25440: F, t25468: F, t25479: F, t25547: F, t25552: F, t2601: F, t2672: F, t2704: F, t2721: F, t2722: F, t2812: F, t3813: F, t3836: F, t3884: F, t3917: F, t7380: F, t7865: F, t7872: F, t8049: F, t8052: F, t8140: F, t8201: F, t894: F, t914: F, t930: F, t953: F) -> (F, F, F, F) {
    let t25561 = t8207 * t25560;
    let t25562 = t864 * t7373;
    let t25564 = t935 * t769;
    let t25565 = t549 * t1;
    let t25566 = t25564 * t25565;
    let t25570 = t3916 * t25560;
    let t25591 = t2270 * t7982;
    let t25595 = t1885 * t2670;
    let t25604 = 0.13613985915860191978e1 * t25547 + 0.10747883617784362088e1 * t2704 * t7872 - 0.30909018630360027928e0 * t25552 + 0.49903344976940985984e3 * t8140 * t8052 + 0.18137053605011111024e0 * t953 * t894 * t7865 * t23825 + 0.45352564237957702055e6 * t25561 * t25562 * t7380 * t25566 - 0.45352564237957702055e6 * t25570 * t25562 * t2672 * t25566 + 0.35163949364965747848e4 * t11526 * t25440 * t2672 * t10977 + 0.389869882632351453e2 * t2812 * t3836 * t25468 + 0.64487301706706172529e0 * t2704 * t8049 + 0.90880810212048753088e1 * t2721 * t2722 * t25479 - 0.23181763972770020945e0 * t930 * t914 * t2601 * t23951 - 0.45440405106024376544e1 * t2721 * t2722 * t25591 - 0.35163949364965747848e4 * t3917 * t11374 * t25595 * t8201 + 0.17581974682482873924e4 * t3884 * t11374 * t25595 * t3813;
    (t25562, t25591, t25595, t25604)
}
