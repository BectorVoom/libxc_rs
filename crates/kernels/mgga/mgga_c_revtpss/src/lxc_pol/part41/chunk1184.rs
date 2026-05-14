//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1184/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1184<F: Float>(t11134: F, t11890: F, t15189: F, t15874: F, t15875: F, t15876: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> (F,) {
    let t19855 = -t11890 - 0.37037037037037037037e-2 * t11134 - 0.74074074074074074074e-2 * t15189 + t15874 - t15875 + t15876 + 0.18518518518518518518e-2 * t18919 - 0.92592592592592592592e-2 * t18906 + 0.33333333333333333333e-1 * t18911 - 0.11111111111111111111e-1 * t18915 - 0.55555555555555555557e-2 * t18924 - 0.50000000000000000001e-1 * t18928 + 0.33333333333333333334e-1 * t18932 + 0.27777777777777777778e-2 * t18934 - 0.55555555555555555555e-2 * t18939 + 0.16666666666666666667e-1 * t18944 - 0.83333333333333333333e-2 * t18948;
    (t19855,)
}
