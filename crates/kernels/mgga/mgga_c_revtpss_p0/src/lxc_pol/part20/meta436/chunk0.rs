//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1643/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1643<F: Float>(t12571: F, t3535: F, t1196: F, t3516: F, t3542: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F) -> (F, F, F) {
    let t44984 = F::cast_from(0.70178683471615754484e1_f64) * t12571 * t3535;
    let t44987 = F::cast_from(0.21053605041484726346e2_f64) * t1196 * t3542 * t3516;
    let t44999 = -F::cast_from(0.26382716049382716049e-1_f64) * t43858 - F::cast_from(0.52765432098765432099e-1_f64) * t43862 - F::cast_from(0.14246666666666666667e0_f64) * t43830 - F::cast_from(0.31659259259259259258e-1_f64) * t43865 + F::cast_from(0.47488888888888888888e-1_f64) * t43832 + F::cast_from(0.11872222222222222222e0_f64) * t43837 - F::cast_from(0.35616666666666666666e-1_f64) * t43871 - F::cast_from(0.47488888888888888888e-1_f64) * t43841 + F::cast_from(0.6411e0_f64) * t43845 + F::cast_from(0.10685e0_f64) * t43877 + F::cast_from(0.14246666666666666667e0_f64) * t43849;
    (t44984, t44987, t44999)
}
