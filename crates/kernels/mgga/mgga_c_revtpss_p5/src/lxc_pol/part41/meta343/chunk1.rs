//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1148/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1148<F: Float>(t15827: F, t4837: F, t1659: F, t3105: F, t1062: F, t4797: F, t1660: F, t3201: F, t1058: F, t4798: F, t15127: F, t15125: F) -> (F, F, F, F, F, F, F) {
    let t15829 = F::cast_from(0.57165357490759649296e-3_f64) * t4837 * t15827;
    let t15830 = t1659 * t3105;
    let t15850 = t4797 * t1062;
    let t15862 = t1660 * t3201;
    let t15865 = F::cast_from(0.28582678745379824648e-3_f64) * t4798 * t1058;
    let t15874 = F::cast_from(0.37037037037037037037e-2_f64) * t15127;
    let t15875 = F::cast_from(0.11111111111111111111e-1_f64) * t15125;
    (t15829, t15830, t15850, t15862, t15865, t15874, t15875)
}
