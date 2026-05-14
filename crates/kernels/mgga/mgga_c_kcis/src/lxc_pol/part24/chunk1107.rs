//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1107/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1107<F: Float>(t1020: F, t8047: F, t96476: F, t1250: F, t20684: F, t251: F, t15573: F, t29126: F, t7788: F, t15171: F, t1662: F, t5310: F, t15534: F, t4621: F, t5281: F, t100152: F, t100235: F, t26955: F, t29117: F, t7772: F, t7791: F, t92730: F, t92749: F, t93023: F, t96868: F) -> (F, F, F, F, F, F) {
    let t100264 = t1020 * t96476 * t8047;
    let t100268 = t20684 * t251 * t1250;
    let t100275 = t15573 * t29126;
    let t100276 = t7788 * t100275;
    let t100280 = t5310 * t15171 * t1662;
    let t100284 = t15534 * t5281 * t4621;
    let t100289 = -0.61905925925925925925e-2 * t100264 - 0.25794135802469135802e-3 * t92730 - 0.11584201388888888889e-3 * t100268 * t7791 - 0.92754700520833333334e-4 * t7772 * t100235 + 0.13913205078125e-3 * t7772 * t100152 - t96868 + 0.11584201388888888889e-3 * t100276 - 0.10306077835648148148e-4 * t92749 + 0.30918233506944444444e-4 * t26955 * t100280 + 0.61836467013888888888e-4 * t26955 * t100284 + 0.23168402777777777778e-3 * t93023 * t29117;
    (t100264, t100268, t100275, t100280, t100284, t100289)
}
