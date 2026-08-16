//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1550/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1550<F: Float>(t15154: F, t2908: F, t141: F, t15158: F, t930: F, t4625: F, t698: F, t4622: F, t15130: F, t15137: F, t15142: F, t15147: F, t15151: F, t15156: F, t15160: F) -> (F, F, F, F, F, F) {
    let t15162 = t2908 * t15154;
    let t15163 = t141 * t15162;
    let t15165 = t930 * t15158;
    let t15166 = t141 * t15165;
    let t15168 = t698 * t4625;
    let t15169 = F::cast_from(0.22076e0_f64) * t15168;
    let t15170 = t698 * t4622;
    let t15172 = t2908 * t15130;
    let t15173 = t141 * t15172;
    let t15175 = -F::cast_from(0.20128333333333333333e0_f64) * t15137 - F::cast_from(0.33547222222222222222e0_f64) * t15142 + F::cast_from(0.12077e1_f64) * t15147 + F::cast_from(0.60385e0_f64) * t15151 + F::cast_from(0.12077e1_f64) * t15156 - F::cast_from(0.181155e1_f64) * t15160 + F::cast_from(0.16557e0_f64) * t15163 - F::cast_from(0.49671e0_f64) * t15166 - t15169 + F::cast_from(0.36793333333333333334e-1_f64) * t15170 - F::cast_from(0.5519e-1_f64) * t15173;
    (t15163, t15166, t15168, t15170, t15173, t15175)
}
