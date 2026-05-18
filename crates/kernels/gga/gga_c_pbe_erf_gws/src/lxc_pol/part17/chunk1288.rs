//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1288/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1288<F: Float>(t1144: F, t13923: F, t859: F, t13911: F, t26958: F, t13917: F, t53156: F, t9333: F, t22336: F, t4002: F, t14667: F, t22263: F, t2409: F, t3066: F, t4385: F, t51569: F, t51815: F, t51825: F, t51827: F, t51829: F, t53915: F, t53925: F, t53930: F, t53936: F, t8734: F, t8793: F) -> F {
    let t53939 = t859 * t1144 * t13923;
    let t53943 = F::new(7.0) / F::new(72.0) * t26958 * t13911;
    let t53945 = t13917 * t53156 * t9333;
    let t53948 = F::new(7.0) / F::new(144.0) * t22336 * t4002;
    let t53949 = -F::new(7.0) / F::new(72.0) * t51815 - t53915 + F::new(35.0) / F::new(108.0) * t51825 + F::new(7.0) / F::new(4608.0) * t51827 + t3066 * t2409 * t8734 * t14667 / F::new(24.0) - F::new(7.0) / F::new(576.0) * t51829 - t53925 / F::new(12.0) - t8793 * t51569 / F::new(16.0) + t53930 / F::new(192.0) - t22263 * t4002 / F::new(48.0) - t53936 / F::new(768.0) + t4385 * t53939 / F::new(96.0) - t53943 + t53945 / F::new(256.0) + t53948;
    t53949
}
