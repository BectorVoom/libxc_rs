//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1593/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1593<F: Float>(t1196: F, t12552: F, t3497: F, t43977: F, t12235: F, t3531: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F) -> (F, F, F) {
    let t43980 = F::cast_from(0.61524113149298439947e4_f64) * t1196 * t12552 * t3497 * t43977;
    let t43982 = F::cast_from(0.14035736694323150897e2_f64) * t3531 * t12235;
    let t43994 = -F::cast_from(0.13734567901234567901e-1_f64) * t43858 - F::cast_from(0.27469135802469135803e-1_f64) * t43862 - F::cast_from(0.74166666666666666668e-1_f64) * t43830 - F::cast_from(0.16481481481481481482e-1_f64) * t43865 + F::cast_from(0.24722222222222222222e-1_f64) * t43832 + F::cast_from(0.61805555555555555555e-1_f64) * t43837 - F::cast_from(0.18541666666666666666e-1_f64) * t43871 - F::cast_from(0.24722222222222222222e-1_f64) * t43841 + F::new(0.33375e0) * t43845 + F::cast_from(0.55625000000000000001e-1_f64) * t43877 + F::cast_from(0.74166666666666666668e-1_f64) * t43849;
    (t43980, t43982, t43994)
}
