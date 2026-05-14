//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1174/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1174<F: Float>(t1957: F, t9988: F, t5218: F, t654: F, t7336: F, t9705: F, t2559: F, t9709: F, t33121: F, t7440: F, t33120: F, t739: F, t7312: F, t7069: F, t7316: F, t9704: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34310 = t9988 * t1957;
    let t34312 = 2.0 * t5218 * t34310;
    let t34313 = t7336 * t654;
    let t34314 = t34313 * t9705;
    let t34316 = t2559 * t654;
    let t34317 = t34316 * t9709;
    let t34319 = t33121 * t7440;
    let t34321 = t739 * t33120;
    let t34322 = t34321 * t7312;
    let t34324 = t7316 * t7069;
    let t34325 = t9704 * t34324;
    (t34310, t34312, t34313, t34314, t34316, t34317, t34319, t34321, t34322, t34324, t34325)
}
