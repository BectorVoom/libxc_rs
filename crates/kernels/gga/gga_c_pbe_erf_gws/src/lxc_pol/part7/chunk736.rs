//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 736/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk736<F: Float>(t1492: F, t751: F, t1497: F, t6032: F, t6036: F, t6039: F, t6043: F, t6049: F, t6050: F, t6053: F, t6058: F, t6059: F) -> F {
    let t6061 = t751 * t1492;
    let t6064 = F::new(0.59871170051273045469e-1) * t751 * t1497;
    let t6065 = -t6032 - t6036 - F::new(0.54655730795145295329e-4) * t6039 - t6043 + t6049 - F::new(0.15965645347006145458e0) * t6050 - t6053 - t6058 + F::new(0.59871170051273045469e-1) * t6059 + F::new(0.11974234010254609094e0) * t6061 + t6064;
    t6065
}
