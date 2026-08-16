//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1974/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1974<F: Float>(t29506: F, t29864: F, t3: F, t1458: F, t24972: F, t27921: F, t28888: F, t28890: F, t28892: F, t28895: F, t28898: F, t28901: F, t28903: F, t5456: F, t5493: F, t577: F, t7423: F) -> (F, F, F) {
    let t29865 = t29506 + t29864;
    let t29866 = t3 * t29865;
    let t29884 = F::cast_from(0.45e1_f64) * t29865 * t577 + F::cast_from(27.0_f64) * t27921 * t1458 + F::cast_from(27.0_f64) * t24972 * t5456 + F::cast_from(0.135e2_f64) * t7423 * t5493 + t28888 + t28890 + t28892 + t28895 + t28898 + t28901 + t28903;
    (t29865, t29866, t29884)
}
