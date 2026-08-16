//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 705/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk705<F: Float>(t225: F, t4376: F, t4407: F, t227: F, t73: F, t1544: F, t853: F, t775: F, t4343: F, t832: F, t1553: F, t1555: F, t229: F, t830: F, t833: F) -> (F, F, F, F, F, F) {
    let t4409 = (t4376 + t4407) * t225;
    let t4415 = t227 * t73;
    let t4416 = t853 * t1544;
    let t4417 = t4416 * t775;
    let t4420 = t832 * t4343;
    let t4423 = F::cast_from(3.0_f64) * t1553 * t833 + F::cast_from(3.0_f64) * t1555 * t830 + F::cast_from(3.0_f64) * t227 * t4420 - t229 * t4409 - F::cast_from(12.0_f64) * t4415 * t4417;
    (t4409, t4415, t4416, t4417, t4420, t4423)
}
