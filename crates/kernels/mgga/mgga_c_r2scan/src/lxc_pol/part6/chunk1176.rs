//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1176/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1176<F: Float>(t21430: F, t225: F, t704: F, t1937: F, t1938: F, t2005: F, t2008: F, t207: F, t21065: F, t21069: F, t21088: F, t21091: F, t21094: F, t21117: F, t21416: F, t21420: F, t390: F, t5317: F, t5549: F, t5694: F, t5697: F, t5747: F, t5823: F, t673: F, t686: F, t689: F, t740: F) -> (F, F) {
    let t21432 = t704 * t21430 * t225;
    let t21440 = -0.123288e1 * t390 * t1937 * t5823 - t21065 + t21069 + 0.69350015718254262348e2 * t21117 + 0.2379258106121766316e3 * t2005 * t21416 * t689 - 0.123288e1 * t673 * t21420 * t207 + 0.41023178511846815777e5 * t5694 * t21416 * t5697 - t21088 - t21091 - 0.51017573446331031809e4 * t5747 * t21416 * t2008 - 0.3903689268108626343e0 * t21432 - 0.5204919024144835124e0 * t704 * t5317 * t740 + 0.26436201179130736844e2 * t686 * t5549 * t1938 - t21094;
    (t21432, t21440)
}
