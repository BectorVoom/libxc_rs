//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1308/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1308<F: Float>(t13776: F, t3975: F, t46392: F, t13781: F, t3222: F, t3886: F, t3972: F, t14733: F, t8700: F, t14113: F, t15204: F, t1144: F, t12201: F, t13772: F, t13939: F, t14577: F, t335: F, t338: F, t3913: F, t3917: F, t3921: F, t4002: F, t51530: F, t53646: F, t53656: F, t56697: F, t56701: F, t56703: F, t56708: F) -> F {
    let t56717 = t13776 * t3975 * t46392;
    let t56722 = t3972 * t13781 * t3886 * param_a_c * t3222;
    let t56724 = t14733 * t8700;
    let t56728 = t14113 * t15204;
    let t56735 = -t56697 / F::new(1536.0) - t56701 / F::new(3072.0) - t53646 + F::new(7.0) / F::new(144.0) * t56703 - t56708 / F::new(384.0) - t12201 * t4002 / F::new(96.0) - t3913 * t13772 / F::new(96.0) - t3917 * t13939 / F::new(96.0) - t56717 / F::new(384.0) - t56722 / F::new(1536.0) + t56724 / F::new(24.0) - t3921 * t13939 / F::new(96.0) + t53656 + F::new(7.0) / F::new(1152.0) * t56728 - F::new(119.0) / F::new(3456.0) * t51530 - t335 * t338 * t1144 * t14577 / F::new(48.0);
    t56735
}
