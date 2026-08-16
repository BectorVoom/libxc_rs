//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 902/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk902<F: Float>(t9775: F, t9777: F, t9780: F, t9783: F, t9789: F, t9791: F, t9793: F, t9796: F, t9800: F, t9802: F, t9805: F, t9808: F, t9811: F) -> F {
    let t10946 = F::cast_from(0.12328882118870421572e-6_f64) * t9775 + F::cast_from(0.9275345110817126956e-4_f64) * t9777 + F::cast_from(0.77294542590142724634e-6_f64) * t9780 - F::cast_from(0.1374296967252737644e-5_f64) * t9783 - F::cast_from(0.56273499301538336858e-8_f64) * t9789 - F::cast_from(0.9275345110817126956e-4_f64) * t9791 + F::cast_from(0.132681342766433194e-5_f64) * t9793 - F::cast_from(0.55603792169291016668e-2_f64) * t9796 - F::cast_from(0.29517957899305555558e-5_f64) * t9800 - F::cast_from(0.2698425785107458272e-5_f64) * t9802 - F::cast_from(0.15176747947735985782e-6_f64) * t9805 + F::cast_from(0.2698425785107458272e-6_f64) * t9808 - F::cast_from(0.57970906942607043472e-5_f64) * t9811;
    t10946
}
