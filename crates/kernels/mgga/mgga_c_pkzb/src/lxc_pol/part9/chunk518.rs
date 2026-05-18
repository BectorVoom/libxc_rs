//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 518/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk518<F: Float>(t133: F, t2009: F, t793: F, t2036: F, t306: F, t2126: F, t287: F, t2124: F, t2111: F, t2123: F, t2128: F, t2131: F, t290: F, t791: F, t794: F) -> (F, F, F, F) {
    let t2134 = t2009 * t133;
    let t2135 = t2134 * t793;
    let t2138 = t2036 * t306;
    let t2139 = t2126 * t287;
    let t2140 = t2124 * t2139;
    let t2145 = F::new(0.13170898365871023197e1) * t2123 * t2128 + F::new(0.13170898365871023197e1) * t2131 * t794 + F::new(0.65854491829355115987e0) * t791 * t2135 - F::new(0.65854491829355115987e0) * t2138 * t2140 + F::new(0.65854491829355115987e0) * t290 * t2111;
    (t2134, t2135, t2140, t2145)
}
