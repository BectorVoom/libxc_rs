//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1167/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1167<F: Float>(t5962: F, t853: F, t775: F, t18392: F, t832: F, t1553: F, t1555: F, t18586: F, t18592: F, t18600: F, t18603: F, t227: F, t229: F, t4409: F, t4415: F, t4417: F, t4420: F, t6006: F, t6010: F, t6013: F, t830: F, t833: F) -> F {
    let t18608 = t853 * t5962;
    let t18609 = t18608 * t775;
    let t18612 = t832 * t18392;
    let t18615 = F::new(6.0) * t1553 * t4420 + F::new(6.0) * t1555 * t4409 - t18586 * t229 - F::new(24.0) * t18592 * t4417 + F::new(60.0) * t18600 * t4415 - F::new(24.0) * t18603 * t4415 - F::new(12.0) * t18609 * t4415 + F::new(3.0) * t18612 * t227 + F::new(3.0) * t6006 * t833 - F::new(12.0) * t6010 * t830 + F::new(3.0) * t6013 * t830;
    t18615
}
