//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1138/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1138<F: Float>(t47184: F, t52112: F, t14849: F, t804: F, t12276: F, t15102: F, t321: F, t1105: F, t13756: F, t14364: F, t14383: F, t14825: F, t15101: F, t2423: F, t3946: F, t4062: F, t4066: F, t52089: F, t52113: F, t52115: F, t52127: F, t8574: F, t8759: F) -> (F,) {
    let t54832 = 6.0 * t52112 * t47184;
    let t54843 = 6.0 * t804 * t14849;
    let t54852 = 6.0 * t52112 * t12276;
    let t54854 = 2.0 * t321 * t15102;
    let t54858 = 3.0 * t1105 * t3946 * t52089 + 6.0 * t13756 * t4066 * t8759 - 6.0 * t14364 * t14383 * t3946 - 6.0 * t14364 * t14825 * t3946 - t15101 * t2423 * t4062 + 3.0 * t3946 * t4066 * t8574 - 6.0 * t52113 - 2.0 * t52115 - t52127 - t54832 + t54843 - t54852 - t54854;
    (t54858,)
}
