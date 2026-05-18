//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 885/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk885<F: Float>(t1716: F, t2936: F, t1035: F, t1836: F, t1024: F, t2060: F, t1025: F, t1030: F, t1841: F, t1850: F, t1897: F, t1935: F, t1939: F, t2508: F, t2928: F, t2951: F, t2964: F, t5288: F, t5293: F, t650: F, t681: F, t7066: F, t8868: F, t8872: F, t8875: F, t8879: F, t8882: F) -> F {
    let t8902 = t2936 * t1716;
    let t8905 = t1035 * t1836;
    let t8908 = t2060 * t1024;
    let t8911 = F::new(0.17090058289204942853e-2) * t1850 * t8868 - F::new(0.17090058289204942853e-2) * t1841 * t8872 - F::new(0.34180116578409885705e-2) * t1841 * t8875 + F::new(0.51270174867614828558e-2) * t1841 * t8879 - F::new(0.17090058289204942853e-2) * t1850 * t8882 - F::new(0.1281754371690370714e-2) * t7066 + F::new(0.20508069947045931424e-1) * t1939 * t1025 + F::new(0.20508069947045931424e-1) * t650 * t2928 + F::new(0.76905262301422242837e-2) * t1935 * t1025 + F::new(0.15381052460284448567e-1) * t681 * t2928 - F::new(0.20508069947045931424e-1) * t1939 * t1030 - F::new(0.20508069947045931424e-1) * t650 * t2964 + F::new(0.15381052460284448567e-1) * t5288 * t2951 + F::new(0.20508069947045931424e-1) * t5293 * t2951 - F::new(0.23071578690426672851e-1) * t2508 * t8902 - F::new(0.76905262301422242837e-2) * t1897 * t8905 + F::new(0.76905262301422242837e-2) * t2508 * t8908;
    t8911
}
