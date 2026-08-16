//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 681/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk681<F: Float>(t1096: F, t1976: F, t7160: F, t3140: F, t378: F, t1078: F, t1982: F, t1035: F, t1043: F, t1089: F, t1984: F, t359: F, t7135: F) -> (F, F, F, F, F) {
    let t7161 = t1976 * t1096;
    let t7162 = t7160 * t7161;
    let t7165 = t378 * t3140;
    let t7166 = t7165 * t1078;
    let t7167 = t1982 * t7166;
    let t7168 = t1035 * t1976;
    let t7170 = t7168 * t1043 * t1089;
    let t7174 = t1984 * t359 * t7135;
    (t7162, t7167, t7168, t7170, t7174)
}
