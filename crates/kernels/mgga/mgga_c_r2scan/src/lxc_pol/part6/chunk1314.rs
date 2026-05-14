//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1314/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1314<F: Float>(t20143: F, t20148: F, t20229: F, t20231: F, t20235: F, t20240: F, t20244: F, t20253: F, t24805: F, t24816: F, t24822: F, t24825: F, t24827: F, t6372: F, t8198: F, t2654: F, t481: F) -> (F, F) {
    let t24829 = -t24805 + 0.69345773920434148506e0 * t20143 - 0.12459097221822660494e0 * t20148 + 0.26004665220162805689e0 * t8198 * t6372 - 0.11524536070137145298e1 * t20229 - 0.25426783770825854452e1 * t20231 - 0.12713391885412927226e1 * t20235 + 0.11426392607441748233e0 * t20240 + 0.34930954652346593433e-1 * t24816 + 0.28914548798370980346e-4 * t24822 + 0.29272321618148349056e-1 * t20244 - t20253 + 0.34930954652346593433e-1 * t24825 + 0.1047928639570397803e0 * t24827;
    let t24831 = t2654 * t481;
    (t24829, t24831)
}
