//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1350/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1350<F: Float>(t12275: F, t14825: F, t3931: F, t810: F, t3703: F, t944: F, t52112: F, t57779: F, t15567: F, t945: F, t12263: F, t13756: F, t14364: F, t14831: F, t15101: F, t15124: F, t3928: F, t3946: F, t4062: F, t4066: F, t4120: F, t54792: F, t56018: F, t56027: F, t57785: F, t9807: F) -> (F, F, F, F) {
    let t57809 = t12275 * t14825;
    let t57820 = t3931 * t810;
    let t57830 = t3703 * t944;
    let t57860 = t52112 * t57779;
    let t57883 = t15567 * t945;
    let t57889 = -t12263 * t4062 * t4120 - F::cast_from(6.0_f64) * t13756 * t4120 * t57830 - F::cast_from(6.0_f64) * t14364 * t15124 * t3946 - t14364 * t3928 * t4062 - F::cast_from(6.0_f64) * t14825 * t15101 * t3946 + F::cast_from(4.0_f64) * t14831 * t4062 * t54792 + F::cast_from(3.0_f64) * t3946 * t4066 * t9807 - F::cast_from(6.0_f64) * t3946 * t4120 * t56018 - F::cast_from(6.0_f64) * t3946 * t4120 * t56027 - F::cast_from(3.0_f64) * t3946 * t4120 * t57785 + F::cast_from(3.0_f64) * t3946 * t57883 * t810 - F::cast_from(6.0_f64) * t57860;
    (t57809, t57820, t57883, t57889)
}
