//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1317/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1317<F: Float>(t3666: F, t7623: F, t12808: F, t29096: F, t3655: F, t7610: F, t1256: F, t26817: F, t12898: F, t2139: F, t12995: F, t26824: F) -> (F, F, F, F, F, F) {
    let t97250 = t3666 * t7623;
    let t97261 = t12808 * t29096;
    let t97267 = t7610 * t3655;
    let t97269 = t26817 * t1256;
    let t97272 = F::cast_from(0.1270341277572436651e-3_f64) * t2139 * t12898;
    let t97279 = t26824 * t12995;
    (t97250, t97261, t97267, t97269, t97272, t97279)
}
