//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 863/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk863<F: Float>(t10935: F, t3446: F, t766: F, t2279: F, t3428: F, t3430: F, t10810: F, t1104: F, t3429: F, t158: F, t607: F, t122: F, t3434: F, t3437: F, t2317: F, t502: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10937 = t3446 * t10935 * t766;
    let t10943 = t2279 * t3428;
    let t10944 = t10943 * t3430;
    let t10945 = 0.30487649791575028314e-3 * t10944;
    let t10946 = t10810 * t1104;
    let t10947 = t3429 * t10946;
    let t10948 = 0.81300399444200075504e-3 * t10947;
    let t10949 = t158 * t607;
    let t10950 = t10949 * t122;
    let t10952 = t3434 * t3437 * t10950;
    let t10954 = t502 * t2317;
    (t10937, t10943, t10945, t10946, t10948, t10949, t10950, t10952, t10954)
}
