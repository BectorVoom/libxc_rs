//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1211/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1211<F: Float>(t22914: F, t25570: F, t22878: F, t6414: F, t100166: F, t101908: F, t103: F, t1286: F, t1557: F, t1564: F, t22865: F, t22873: F, t22904: F, t22917: F, t22935: F, t23133: F, t25558: F, t25577: F, t25605: F, t25615: F, t25618: F, t26128: F, t28: F, t3052: F, t3188: F, t497: F, t5501: F, t6457: F, t925: F, t93866: F, t93882: F, t94038: F) -> (F,) {
    let t101922 = t22914 * t25570 / 27.0;
    let t101932 = 2.0 / 9.0 * t6414 * t22878;
    let t101938 = t25558 * t22904 / 9.0 - 2.0 * t100166 - 2.0 / 3.0 * t1286 * t28 * t22873 * t26128 + 2.0 * t101908 * t103 - 2.0 / 27.0 * t5501 * t25615 * t497 * t1557 * t3188 - 2.0 / 27.0 * t93882 + t23133 * t6457 / 6.0 + t6414 * t22865 / 6.0 + t101922 - t5501 * t1564 * t93866 * t925 / 9.0 - 2.0 / 9.0 * t25577 * t1564 * t22917 * t3052 + t101932 - 2.0 / 27.0 * t22935 * t25618 + 2.0 / 9.0 * t5501 * t94038 * t25605;
    (t101938,)
}
