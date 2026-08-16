//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 885/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk885(t1716: f64, t2936: f64, t1035: f64, t1836: f64, t1024: f64, t2060: f64, t1025: f64, t1030: f64, t1841: f64, t1850: f64, t1897: f64, t1935: f64, t1939: f64, t2508: f64, t2928: f64, t2951: f64, t2964: f64, t5288: f64, t5293: f64, t650: f64, t681: f64, t7066: f64, t8868: f64, t8872: f64, t8875: f64, t8879: f64, t8882: f64) -> f64 {
    let t8902 = t2936 * t1716;
    let t8905 = t1035 * t1836;
    let t8908 = t2060 * t1024;
    let t8911 = 0.17090058289204942853e-2_f64 * t1850 * t8868 - 0.17090058289204942853e-2_f64 * t1841 * t8872 - 0.34180116578409885705e-2_f64 * t1841 * t8875 + 0.51270174867614828558e-2_f64 * t1841 * t8879 - 0.17090058289204942853e-2_f64 * t1850 * t8882 - 0.1281754371690370714e-2_f64 * t7066 + 0.20508069947045931424e-1_f64 * t1939 * t1025 + 0.20508069947045931424e-1_f64 * t650 * t2928 + 0.76905262301422242837e-2_f64 * t1935 * t1025 + 0.15381052460284448567e-1_f64 * t681 * t2928 - 0.20508069947045931424e-1_f64 * t1939 * t1030 - 0.20508069947045931424e-1_f64 * t650 * t2964 + 0.15381052460284448567e-1_f64 * t5288 * t2951 + 0.20508069947045931424e-1_f64 * t5293 * t2951 - 0.23071578690426672851e-1_f64 * t2508 * t8902 - 0.76905262301422242837e-2_f64 * t1897 * t8905 + 0.76905262301422242837e-2_f64 * t2508 * t8908;
    t8911
}
