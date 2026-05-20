//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2855/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2855<F: Float>(t23121: F, t40188: F, t40121: F, t40132: F, t40139: F, t40088: F, t40099: F, t40103: F, t40115: F, t40131: F, t40137: F, t50048: F, t76986: F, t76987: F, t76988: F, t76991: F, t76992: F, t76995: F) -> (F, F, F, F, F) {
    let t76997 = F::new(24.0) * t40188 * t23121;
    let t76998 = F::cast_from(0.10389515463408878255e3_f64) * t40121;
    let t76999 = F::cast_from(0.5848223622634646207e0_f64) * t40132;
    let t77000 = F::new(4.0) * t40139;
    let t77001 = t76986 + t40088 - t76987 + t76988 + t40099 + t40103 + t76991 + t50048 + t76992 + t76995 + t76997 - t40115 + t76998 - t40131 - t76999 - t40137 + t77000;
    (t76997, t76998, t76999, t77000, t77001)
}
