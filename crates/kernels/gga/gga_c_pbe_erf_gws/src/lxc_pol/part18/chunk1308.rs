//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1308/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1308<F: Float>(t13776: F, t3975: F, t46392: F, t13781: F, t3222: F, t3886: F, t3972: F, t14733: F, t8700: F, t14113: F, t15204: F, t1144: F, t12201: F, t13772: F, t13939: F, t14577: F, t335: F, t338: F, t3913: F, t3917: F, t3921: F, t4002: F, t51530: F, t53646: F, t53656: F, t56697: F, t56701: F, t56703: F, t56708: F, param_a_c: F) -> F {
    let t56717 = t13776 * t3975 * t46392;
    let t56722 = t3972 * t13781 * t3886 * param_a_c * t3222;
    let t56724 = t14733 * t8700;
    let t56728 = t14113 * t15204;
    let t56735 = -t56697 / F::cast_from(1536.0_f64) - t56701 / F::cast_from(3072.0_f64) - t53646 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t56703 - t56708 / F::cast_from(384.0_f64) - t12201 * t4002 / F::cast_from(96.0_f64) - t3913 * t13772 / F::cast_from(96.0_f64) - t3917 * t13939 / F::cast_from(96.0_f64) - t56717 / F::cast_from(384.0_f64) - t56722 / F::cast_from(1536.0_f64) + t56724 / F::cast_from(24.0_f64) - t3921 * t13939 / F::cast_from(96.0_f64) + t53656 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t56728 - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t51530 - t335 * t338 * t1144 * t14577 / F::cast_from(48.0_f64);
    t56735
}
