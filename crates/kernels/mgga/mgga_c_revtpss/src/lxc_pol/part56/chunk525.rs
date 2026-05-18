//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 525/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk525<F: Float>(t225: F, t4376: F, t4407: F, t227: F, t73: F, t1544: F, t853: F, t775: F, t4343: F, t832: F, t1553: F, t1555: F, t229: F, t830: F, t833: F) -> F {
    let t4409 = (t4376 + t4407) * t225;
    let t4415 = t227 * t73;
    let t4416 = t853 * t1544;
    let t4417 = t4416 * t775;
    let t4420 = t832 * t4343;
    let t4423 = F::new(3.0) * t1553 * t833 + F::new(3.0) * t1555 * t830 + F::new(3.0) * t227 * t4420 - t229 * t4409 - F::new(12.0) * t4415 * t4417;
    t4423
}
