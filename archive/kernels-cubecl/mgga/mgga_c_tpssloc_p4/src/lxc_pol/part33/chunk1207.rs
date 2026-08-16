//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1207/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1207<F: Float>(t28896: F, t3941: F, t1873: F, t5493: F, t1401: F, t28017: F, t1458: F, t23880: F, t26523: F, t28868: F, t28888: F, t28890: F, t28892: F, t28895: F, t5456: F, t577: F, t7010: F) -> (F, F) {
    let t28898 = F::cast_from(54.0_f64) * t3941 * t28896;
    let t28899 = t1873 * t5493;
    let t28901 = F::cast_from(27.0_f64) * t3941 * t28899;
    let t28903 = F::cast_from(0.135e2_f64) * t1401 * t28017;
    let t28904 = F::cast_from(0.45e1_f64) * t28868 * t577 + F::cast_from(27.0_f64) * t26523 * t1458 + F::cast_from(27.0_f64) * t23880 * t5456 + F::cast_from(0.135e2_f64) * t7010 * t5493 + t28888 + t28890 + t28892 + t28895 + t28898 + t28901 + t28903;
    (t28899, t28904)
}
