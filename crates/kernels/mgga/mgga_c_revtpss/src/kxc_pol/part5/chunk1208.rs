//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1208/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1208<F: Float>(t6152: F, t945: F, t15170: F, t15189: F, t15312: F, t15322: F, t15324: F, t18944: F, t18961: F, t18964: F, t18967: F, t18970: F, t18973: F) -> (F, F) {
    let t19173 = t6152 * t945;
    let t19202 = F::new(0.103295e1) * t18944 + F::new(0.20839e0) * t18961 - F::cast_from(0.69463333333333333334e-1_f64) * t18964 - F::cast_from(0.46308888888888888889e-1_f64) * t18967 - F::new(0.62517e0) * t18970 + F::new(0.41678e0) * t18973 - t15312 + F::cast_from(0.4630888888888888889e-1_f64) * t15170 - F::cast_from(0.45908888888888888888e0_f64) * t15189 + t15322 + t15324;
    (t19173, t19202)
}
