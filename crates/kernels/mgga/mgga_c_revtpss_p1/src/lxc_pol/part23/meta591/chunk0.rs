//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2228/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2228<F: Float>(t23740: F, t23753: F, t954: F, t1621: F, t19275: F, t1634: F, t6205: F, t1633: F, t19303: F, t1610: F, t6141: F, t2874: F) -> (F, F, F, F, F, F, F) {
    let t23754 = t23740 + t23753;
    let t23755 = t23754 * t954;
    let t23758 = t19275 * t1621;
    let t23761 = t1634 * t6205;
    let t23764 = t19303 * t1633;
    let t23767 = t1610 * t6141;
    let t23769 = F::new(6.0) * t2874 * t23767;
    (t23754, t23755, t23758, t23761, t23764, t23767, t23769)
}
