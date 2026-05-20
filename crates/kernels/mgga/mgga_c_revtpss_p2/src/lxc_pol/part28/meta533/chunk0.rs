//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1975/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1975<F: Float>(t1497: F, t640: F, t77: F, t4241: F, t84: F, t1470: F, t2242: F, t1923: F, t1928: F, t25106: F, t28078: F, t28081: F, t28086: F, t28090: F, t28093: F, t6954: F, t6958: F, t6974: F, t6978: F, t7702: F, t7706: F, t7716: F, t7720: F) -> (F, F, F, F, F, F) {
    let t28104 = t640 * t1497;
    let t28105 = t77 * t28104;
    let t28108 = t84 * t4241;
    let t28109 = t77 * t28108;
    let t28112 = t2242 * t1470;
    let t28115 = -t1923 * t28078 / F::new(6.0) - t1923 * t28081 / F::new(6.0) - t6954 * t7720 / F::new(6.0) - t1923 * t28086 / F::new(6.0) - t1923 * t28090 / F::new(6.0) - t28093 * t1928 / F::new(6.0) - t7702 * t6974 / F::new(6.0) - t7702 * t6978 / F::new(6.0) - t6954 * t7716 / F::new(6.0) + F::new(5.0) / F::new(6.0) * t25106 * t7706 + F::new(5.0) / F::new(6.0) * t6958 * t28105 + F::new(5.0) / F::new(6.0) * t6958 * t28109 + t28112 * t1928 / F::new(3.0);
    (t28104, t28105, t28108, t28109, t28112, t28115)
}
