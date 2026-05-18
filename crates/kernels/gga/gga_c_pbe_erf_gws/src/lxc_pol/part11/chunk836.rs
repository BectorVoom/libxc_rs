//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 836/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk836<F: Float>(t11447: F, t11782: F, t3134: F, t1105: F, t337: F, t3791: F, t2147: F, t3116: F, t11787: F, t9035: F, t3763: F, t3781: F) -> (F, F, F, F, F, F, F) {
    let t13238 = F::new(7.0) / F::new(48.0) * t11447;
    let t13240 = t11782 * t3134 / F::new(32.0);
    let t13242 = t337 * t3791 * t1105;
    let t13243 = t2147 * t13242;
    let t13245 = t3116 * t13243 / F::new(16.0);
    let t13247 = t9035 * t11787 / F::new(16.0);
    let t13248 = t3781 * t3763;
    (t13238, t13240, t13242, t13243, t13245, t13247, t13248)
}
