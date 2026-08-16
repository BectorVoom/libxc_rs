//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 873/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk873<F: Float>(t20947: F, t2701: F, t820: F, t20870: F, t819: F, t13283: F, t1512: F, t1516: F, t16872: F, t16976: F, t20904: F, t20908: F, t20938: F, t20944: F, t249: F, t4172: F, t5587: F, t5624: F, t5628: F, t817: F, t843: F, t9559: F, t9974: F) -> (F, F, F) {
    let t20949 = t2701 * t820 * t20947;
    let t20953 = t819 * t820 * t20870;
    let t20958 = -t9974 * t20904 / F::cast_from(512.0_f64) - t843 * t20908 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t4172 * t5624 - t16976 * t1516 / F::cast_from(256.0_f64) - t4172 * t5628 / F::cast_from(256.0_f64) + t20938 * t249 / F::cast_from(3072.0_f64) + t13283 * t5587 / F::cast_from(512.0_f64) - t9559 * t20944 / F::cast_from(4.0_f64) + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t843 * t20949 - t817 * t20953 / F::cast_from(3072.0_f64) - t16872 * t1512 / F::cast_from(1024.0_f64);
    (t20949, t20953, t20958)
}
