//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 815/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk815<F: Float>(t703: F, t820: F, t684: F, t1701: F, t2735: F, t6027: F, t2719: F, t1472: F, t14721: F, t14729: F, t14742: F, t14766: F, t24291: F, t25049: F, t25050: F, t25055: F, t25060: F, t25064: F, t25070: F, t25072: F, t25077: F, t4099: F, t4104: F, t6035: F, t6045: F, t6256: F) -> (F, F, F) {
    let t25078 = t703 * t820;
    let t25079 = t25078 * t684;
    let t25088 = t1701 * t6027 * t2735;
    let t25092 = t1701 * t6027 * t2719;
    let t25097 = 0.40006800655555555556e0 * t25049 * t6045 * t25050 - 0.66678001092592592595e-1 * t25055 + 0.90613700826057446696e0 * t14766 * t25060 - 0.45306850413028723348e0 * t14729 * t25064 + 0.33339000546296296298e-1 * t6256 * t24291 - 0.66678001092592592595e-1 * t25070 * t6035 * t25072 + 0.66678001092592592595e-1 * t25077 * t6035 * t25079 + 0.45306850413028723348e0 * t14742 * t25064 - 0.90613700826057446696e0 * t14721 * t25060 + 0.22653425206514361674e0 * t1472 * t25088 - 0.45306850413028723348e0 * t4104 * t25092 - 0.22653425206514361674e0 * t4099 * t25088;
    (t25079, t25092, t25097)
}
