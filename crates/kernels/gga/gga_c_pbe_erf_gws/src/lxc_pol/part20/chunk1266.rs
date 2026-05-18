//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1266/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1266<F: Float>(t54305: F, t54352: F, t54356: F, t54381: F, t54427: F, t54621: F, t54641: F, t54719: F, t54724: F, t1167: F, t2494: F, t1105: F, t3324: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t55582 = F::new(119.0) / F::new(1728.0) * t54305;
    let t55607 = F::new(119.0) / F::new(864.0) * t54352;
    let t55609 = F::new(35.0) / F::new(108.0) * t54356;
    let t55623 = F::new(35.0) / F::new(216.0) * t54381;
    let t55751 = F::new(119.0) / F::new(1728.0) * t54427;
    let t55892 = F::new(35.0) / F::new(216.0) * t54621;
    let t55947 = F::new(35.0) / F::new(216.0) * t54641;
    let t55984 = F::new(35.0) / F::new(108.0) * t54719;
    let t55986 = F::new(119.0) / F::new(6912.0) * t54724;
    let t56018 = t2494 * t1167;
    let t56027 = t1105 * t3324;
    (t55582, t55607, t55609, t55623, t55751, t55892, t55947, t55984, t55986, t56018, t56027)
}
