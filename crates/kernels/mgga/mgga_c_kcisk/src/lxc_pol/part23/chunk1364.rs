//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1364/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1364<F: Float>(t12829: F, t1328: F, t1339: F, t32203: F, t5621: F, t18949: F, t9461: F, t19742: F, t5600: F, t33383: F, t3969: F, t32042: F, t33384: F, t109699: F, t109701: F, t110384: F, t110423: F, t110466: F, t110635: F, t113815: F, t113821: F, t18953: F, t19033: F, t20111: F, t32008: F, t32087: F, t33415: F, t33417: F, t33422: F, t3583: F, t3937: F, t6175: F, t9449: F) -> (F, F, F, F, F) {
    let t114038 = t1328 * t12829;
    let t114051 = t1339 * t32203 * t5621;
    let t114054 = t1339 * t9461 * t18949;
    let t114057 = t5600 * t9461 * t19742;
    let t114059 = t33383 * t3969;
    let t114062 = t33384 * t32042;
    let t114064 = 0.13402777777777777778e-2 * t32008 * t113815 - 0.77602083333333333335e-3 * t110635 * t113821 - 0.92592592592592592594e-2 * t110423 * t33417 - 0.92592592592592592594e-2 * t110384 * t33417 - 0.46296296296296296297e-2 * t32087 * t6175 * t33415 * t18953 - 0.10802469135802469136e-1 * t32087 * t20111 * t114038 * t19033 + 0.34722222222222222223e-2 * t32087 * t3937 * t33422 * t3583 - 0.22109259259259259258e-2 * t109699 - 0.73697530864197530861e-3 * t109701 - 0.34722222222222222223e-2 * t110466 + 0.22109259259259259258e-2 * t114051 + 0.11054629629629629629e-2 * t114054 + 0.44218518518518518517e-2 * t114057 + 0.18518518518518518519e-1 * t114059 * t9449 - 0.23148148148148148148e-2 * t114062;
    (t114051, t114054, t114057, t114059, t114064)
}
