//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 261/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk261<F: Float>(t1143: F, t240: F, t462: F, t1122: F, t141: F, t1124: F, t1135: F, t1137: F, t1140: F, t421: F, t1131: F, t1118: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1144 = F::cast_from(0.82156666666666666667e-1_f64) * t1143;
    let t1145 = t240 * t462;
    let t1146 = t1145 * t1122;
    let t1147 = t141 * t1146;
    let t1149 = F::new(0.1898925e1) * t1135 - t1137 + F::cast_from(0.29896666666666666667e0_f64) * t1124 + F::new(0.3071625e0) * t1140 - t1144 + F::cast_from(0.82156666666666666667e-1_f64) * t1147;
    let t1150 = F::new(1.0) / t421;
    let t1151 = t1149 * t1150;
    let t1153 = F::new(1.0) * t1131 * t1151;
    let t1154 = F::cast_from(0.17123333333333333333e-1_f64) * t1118;
    (t1144, t1145, t1146, t1147, t1149, t1150, t1151, t1153, t1154)
}
