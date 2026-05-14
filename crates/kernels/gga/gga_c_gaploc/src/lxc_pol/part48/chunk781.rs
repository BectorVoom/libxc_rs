//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 781/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk781<F: Float>(t10930: F, t10931: F, t45320: F, t44888: F, t723: F, t1457: F, t2645: F, t36516: F, t43464: F, t43467: F, t43470: F, t10889: F, t326: F, t43476: F, t43502: F, t43519: F, t44878: F, t45315: F, t45319: F, t45323: F, t45326: F, t45329: F, t45331: F, t45335: F, t45337: F, t45343: F, t701: F, t7572: F, t7573: F, t7584: F, t7585: F, t825: F) -> (F, F) {
    let t45349 = 0.55213813373645879534e2 * t10930 * t10931 * t45320;
    let t45350 = t44888 * t723;
    let t45356 = 0.42900587942220512003e1 * t36516 * t1457 * t2645;
    let t45357 = 0.11916829983950142223e0 * t43464;
    let t45358 = 0.11916829983950142223e0 * t43467;
    let t45359 = 0.11916829983950142223e0 * t43470;
    let t45363 = -0.71500979903700853338e0 * t10889 * t1457 * t44878 * t701 + t45315 + t45319 - t45323 + t45326 + 0.12780975317973583226e0 * t45329 + 0.95857314884801874192e0 * t45331 - 0.21301625529955972043e0 * t45335 - 0.23005755572352449806e2 * t7584 * t7585 * t45337 - t45343 + 0.13803453343411469884e2 * t7572 * t7573 * t45337 + t45349 - 0.18404604457881959845e2 * t825 * t326 * t45350 - t45356 + t45357 + t45358 + t45359 - 0.12780975317973583226e0 * t43476 + 0.59584149919750711116e-1 * t43502 + 0.11916829983950142223e0 * t43519;
    (t45350, t45363)
}
