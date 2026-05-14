//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 733/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk733<F: Float>(t7313: F, t732: F, t1877: F, t481: F, t941: F, t2042: F, t2558: F, t943: F, t1841: F, t1850: F, t1897: F, t2508: F, t2538: F, t2573: F, t5227: F, t5288: F, t5293: F, t5524: F, t7251: F, t7255: F, t7260: F, t7268: F, t7277: F, t7281: F, t7286: F, t7293: F, t7299: F, t7303: F, t7306: F, t7309: F) -> (F, F, F) {
    let t7314 = t732 * t7313;
    let t7315 = t481 * t941 * t1877 * t7314;
    let t7317 = t2042 * t2558;
    let t7318 = t943 * t7317;
    let t7320 = 0.15381052460284448567e-1 * t1897 * t7251 + 0.76905262301422242837e-2 * t1897 * t7255 + 0.30762104920568897134e-1 * t2508 * t7260 + 0.20508069947045931424e-1 * t5293 * t2573 + 0.15381052460284448567e-1 * t5288 * t2573 + 0.17090058289204942853e-2 * t1850 * t7268 + 0.8545029144602471425e-3 * t5524 * t2538 - 0.17090058289204942853e-2 * t5227 * t2538 - 0.17090058289204942853e-2 * t1841 * t7277 - 0.17090058289204942853e-2 * t1841 * t7281 + 0.51270174867614828558e-2 * t1841 * t7286 - 0.34180116578409885705e-2 * t1841 * t7293 + 0.64087718584518535698e-3 * t7299 - 0.96131577876777803547e-3 * t7303 - 0.30762104920568897134e-1 * t1897 * t7306 - 0.64087718584518535698e-3 * t7309 + 0.64087718584518535698e-3 * t7315 + 0.32043859292259267849e-3 * t7318;
    (t7315, t7318, t7320)
}
