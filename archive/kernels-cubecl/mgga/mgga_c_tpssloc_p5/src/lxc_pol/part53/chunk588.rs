//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 588/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk588<F: Float>(t3: F, t5363: F, t112: F, t1851: F, t1458: F, t671: F, t1401: F, t3938: F, t3941: F, t4072: F, t577: F, t2235: F, t33: F) -> (F, F, F, F, F) {
    let t5364 = t3 * t5363;
    let t5371 = t1851 * t112;
    let t5376 = t1458 * t671;
    let t5381 = F::cast_from(0.45e1_f64) * t5363 * t577 + F::cast_from(0.135e2_f64) * t5371 * t671 + F::cast_from(0.135e2_f64) * t3938 * t1458 + F::cast_from(27.0_f64) * t3941 * t5376 + F::cast_from(0.135e2_f64) * t1401 * t4072;
    let t6486 = t2235 * t33;
    (t5364, t5371, t5376, t5381, t6486)
}
