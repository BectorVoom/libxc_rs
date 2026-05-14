//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1355/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1355<F: Float>(t1632: F, t5136: F, t551: F, t9981: F, t481: F, t9880: F, t2837: F, t5103: F, t9399: F, t1592: F, t2196: F, t24028: F, t24090: F, t24756: F, t24759: F, t29175: F, t29179: F, t29181: F, t29185: F, t29196: F, t29207: F, t3183: F, t552: F, t560: F, t9190: F) -> (F, F) {
    let t33103 = t5136 * t551 * t1632 * t9981;
    let t33117 = t9880 * t481;
    let t33124 = t5103 * t2837 * t9399;
    let t33128 = 0.69345773920434148507e0 * t33103 - t24756 - t24759 - 0.43371823197556470519e-3 * t29175 + 0.11426392607441748233e0 * t29179 + 0.34672886960217074253e0 * t29181 - 0.10401866088065122276e1 * t29185 - 0.7801399566048841707e1 * t24028 * t9190 - 0.19043987679069580389e-1 * t29196 + 0.13002332610081402845e0 * t1592 * t551 * t552 * t9880 * t560 + 0.5200933044032561138e0 * t2196 * t551 * t552 * t33117 - 0.4939086887201633699e-1 * t29207 + 0.34930954652346593433e-1 * t33124 + 0.15602799132097683414e1 * t24090 * t3183;
    (t33117, t33128)
}
