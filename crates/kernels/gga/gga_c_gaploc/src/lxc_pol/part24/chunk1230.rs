//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1230/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1230<F: Float>(t34635: F, t10431: F, t7014: F, t10435: F, t10525: F, t2365: F, t25723: F, t10514: F, t21370: F, t10531: F, t10534: F, t1406: F, t10557: F, t6795: F, t8072: F, t9285: F) -> (F, F, F, F, F, F, F, F) {
    let t34636 = 0.19171462976960374838e0 * t34635;
    let t34637 = t7014 * t10431;
    let t34638 = 0.38342925953920749676e0 * t34637;
    let t34639 = t7014 * t10435;
    let t34640 = 0.85206502119823888168e-1 * t34639;
    let t34642 = t10525 * t2365 * t25723;
    let t34643 = 0.89376224879626066674e-1 * t34642;
    let t34645 = 0.12423108009070322895e3 * t21370 * t10514;
    let t34648 = 0.55213813373645879534e2 * t1406 * t10531 * t10534;
    let t34650 = 0.42900587942220512003e1 * t10557 * t6795;
    let t34652 = 0.71500979903700853338e0 * t9285 * t8072;
    (t34636, t34638, t34640, t34643, t34645, t34648, t34650, t34652)
}
