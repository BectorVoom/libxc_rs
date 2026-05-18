//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1351/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1351<F: Float>(t15400: F, t804: F, t15577: F, t321: F, t15567: F, t2053: F, t15574: F, t57883: F, t1105: F, t13756: F, t14161: F, t14368: F, t14852: F, t3189: F, t3717: F, t3946: F, t4062: F, t52105: F, t54766: F, t54797: F, t54809: F, t54811: F, t56038: F, t57820: F, t944: F) -> F {
    let t57890 = t804 * t15400;
    let t57895 = t321 * t15577;
    let t57902 = t15567 * t2053;
    let t57911 = t321 * t15574;
    let t57913 = t321 * t57883;
    let t57914 = F::new(6.0) * t1105 * t3946 * t54766 + F::new(12.0) * t13756 * t14852 * t3189 + F::new(3.0) * t14161 * t3717 * t3946 + F::new(6.0) * t14368 * t3946 * t57820 - F::new(6.0) * t4062 * t52105 * t56038 - t4062 * t57902 * t944 + t54797 - t54809 + t54811 + F::new(6.0) * t57890 - t57895 + F::new(2.0) * t57911 + t57913;
    t57914
}
