//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1323/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1323<F: Float>(t2097: F, t2649: F, t571: F, t6214: F, t20407: F, t5146: F, t2545: F, t494: F, t6194: F, t18786: F, t18839: F, t18843: F, t18855: F, t18869: F, t18872: F, t18875: F, t18878: F, t23320: F, t23321: F, t23685: F, t23694: F, t23696: F, t23697: F, t23698: F, t23699: F) -> (F, F, F, F) {
    let t24994 = t571 * t2649 * t2097;
    let t24995 = t24994 * t6214;
    let t24996 = 0.19043987679069580388e-1 * t24995;
    let t24997 = t5146 * t20407;
    let t24999 = t2545 * t494 * t6194;
    let t25000 = t24997 * t24999;
    let t25001 = 0.48787202696913915093e-3 * t25000;
    let t25020 = -t18786 + t23320 + t23321 - t18839 + t18843 - t23685 - t18855 + t23694 - t23696 - t23697 - t23698 - t23699 - t18869 + t18872 + t18875 + t18878;
    (t24996, t24999, t25001, t25020)
}
