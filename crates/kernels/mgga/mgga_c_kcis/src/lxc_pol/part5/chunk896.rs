//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 896/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk896<F: Float>(t373: F, t9587: F, t1164: F, t3225: F, t334: F, t369: F, t86: F, t1143: F, t245: F, t1157: F, t752: F, t1071: F, t1083: F, t2844: F, t1160: F, t318: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t10513 = t373 * t9587;
    let t10525 = t1164 * t3225;
    let t10526 = t10525 * sigma0;
    let t10541 = 0.11791604938271604938e-1 * t86 * t334 * t369;
    let t10544 = t1143 * t245;
    let t10556 = t752 * t1157;
    let t10560 = t1083 * t1071;
    let t10583 = t1083 * t2844;
    let t10631 = t86 * t318 * t1160;
    (t10513, t10526, t10541, t10544, t10556, t10560, t10583, t10631)
}
