//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 725/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk725<F: Float>(t1865: F, t2667: F, t7226: F, t2717: F, t702: F, t1836: F, t954: F, t2060: F, t937: F, t2532: F, t779: F, t1710: F, t2581: F, t2580: F, t1841: F, t1897: F, t2504: F, t2508: F, t2509: F, t2577: F, t5227: F, t5288: F, t5293: F, t5524: F, t7129: F, t7137: F, t7204: F, t7207: F, t7212: F, t7215: F, t7223: F) -> (F, F, F) {
    let t7227 = t2667 * t1865;
    let t7228 = t7226 * t7227;
    let t7233 = t2717 * t702;
    let t7236 = t954 * t1836;
    let t7239 = t2060 * t937;
    let t7242 = t779 * t2532;
    let t7245 = t2581 * t1710;
    let t7246 = t2580 * t7245;
    let t7249 = -0.20508069947045931424e-1 * t5293 * t2504 + 0.20508069947045931424e-1 * t7137 * t2509 - 0.15381052460284448567e-1 * t5288 * t2504 - 0.23071578690426672851e-1 * t2508 * t7204 - 0.85450291446024714264e-3 * t7207 + 0.32043859292259267849e-3 * t7212 + 0.64087718584518535698e-3 * t7215 - 0.8545029144602471425e-3 * t5524 * t2577 + 0.17090058289204942853e-2 * t5227 * t2577 + 0.17090058289204942853e-2 * t1841 * t7223 - 0.46143157380853345701e-1 * t2508 * t7228 + 0.15381052460284448567e-1 * t7129 * t2509 - 0.15381052460284448567e-1 * t1897 * t7233 - 0.76905262301422242837e-2 * t1897 * t7236 + 0.76905262301422242837e-2 * t2508 * t7239 + 0.15381052460284448567e-1 * t2508 * t7242 + 0.15381052460284448567e-1 * t2508 * t7246;
    (t7227, t7245, t7249)
}
