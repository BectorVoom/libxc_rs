//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1028/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1028<F: Float>(t552: F, t9880: F, t551: F, t566: F, t6164: F, t6260: F, t7598: F, t9227: F, t9229: F, t9233: F, t9237: F, t9240: F, t9244: F, t9248: F, t9251: F, t9270: F, t9274: F, t9294: F, t9298: F) -> (F, F) {
    let t10183 = t552 * t9880;
    let t10184 = t551 * t10183;
    let t10195 = 0.34672886960217074253e0 * t9227 + 0.69345773920434148506e0 * t9229 - 0.20803732176130244552e1 * t9233 - 0.49390868872016336989e-1 * t9237 - 0.34672886960217074253e0 * t9240 - t6164 - 0.13002332610081402845e0 * t566 * t10184 + 0.34930954652346593433e-1 * t9244 + 0.1047928639570397803e0 * t9248 + 0.58544643236296698111e-1 * t9251 - 0.12713391885412927226e1 * t7598 + 0.17465477326173296717e-1 * t9270 + 0.16463622957338778996e-1 * t9274 - 0.17465477326173296717e-1 * t9294 + 0.34930954652346593433e-1 * t9298 - t6260;
    (t10184, t10195)
}
