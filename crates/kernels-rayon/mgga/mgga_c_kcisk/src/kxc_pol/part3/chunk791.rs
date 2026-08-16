//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 791/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk791(t772: f64, t12215: f64, t1775: f64, t12143: f64, t2021: f64, t1586: f64, t12163: f64, t12166: f64, t12171: f64, t12175: f64, t12180: f64, t12183: f64, t12186: f64, t12188: f64, t12195: f64, t12200: f64, t12205: f64, t12209: f64, t2013: f64, t2016: f64, t5471: f64, t5488: f64, t5494: f64, t5499: f64, t5503: f64, t782: f64) -> f64 {
    let t783 = 0.0_f64 < t772;
    let t12216 = t1775 * t12215;
    let t12220 = piecewise3(t783, t12143, -t12143);
    let t12221 = t2021 * t12220;
    let t12222 = t1586 * t12221;
    let t12225 = -0.17990788716177317214e-1_f64 * t12163 + 0.53972366148531951639e-1_f64 * t2013 * t12166 + 0.27985671336275826777e-1_f64 * t2013 * t12171 - 0.17990788716177317214e-1_f64 * t12175 - 0.53972366148531951639e-1_f64 * t5471 * t5499 - 0.59969295720591057378e-2_f64 * t12180 + 0.89953943580886586067e-2_f64 * t12183 + 0.11993859144118211476e-1_f64 * t12186 + 0.17990788716177317213e-1_f64 * t12188 + 0.2698618307426597582e-1_f64 * t5471 * t5503 + 0.35981577432354634427e-1_f64 * t5471 * t5488 + 0.2698618307426597582e-1_f64 * t12195 * t2016 - 0.71963154864709268855e-1_f64 * t2013 * t12200 + 0.16191709844559585492e0_f64 * t2013 * t12205 + 0.89953943580886586067e-2_f64 * t2013 * t12209 - 0.53972366148531951639e-1_f64 * t5471 * t5494 - 0.2698618307426597582e-1_f64 * t2013 * t12216 - 0.2698618307426597582e-1_f64 * t782 * t12222;
    t12225
}
