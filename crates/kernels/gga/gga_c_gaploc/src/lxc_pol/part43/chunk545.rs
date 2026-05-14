//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 545/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk545<F: Float>(t10440: F, t1407: F, t3396: F, t10430: F, t912: F, t587: F, t2293: F, t2854: F, t1445: F, t1562: F, t3354: F, t4673: F, t1572: F, t3384: F, t4950: F, t10140: F, t1457: F) -> (F, F, F, F, F, F, F) {
    let t10441 = 0.42603251059911944084e-1 * t10440;
    let t10442 = t1407 * t3396;
    let t10443 = 0.19171462976960374838e0 * t10442;
    let t10444 = t912 * t10430;
    let t10445 = t587 * t10444;
    let t10446 = 0.19171462976960374838e0 * t10445;
    let t10447 = t2854 * t2293;
    let t10448 = t1445 * t10447;
    let t10450 = 0.69017266717057349418e1 * t1562 * t10448;
    let t10455 = t4673 * t3354;
    let t10457 = 0.47667319935800568892e0 * t1572 * t10455;
    let t10459 = 0.71500979903700853338e0 * t4950 * t3384;
    let t10463 = t1457 * t10140;
    (t10441, t10443, t10446, t10450, t10457, t10459, t10463)
}
