//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1381/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1381<F: Float>(t1592: F, t1632: F, t551: F, t8057: F, t2196: F, t7591: F, t26175: F, t26178: F, t26180: F, t26183: F, t26188: F, t26191: F, t26193: F, t26196: F, t26198: F, t26201: F, t6200: F, t6465: F, t7313: F, t8119: F) -> (F,) {
    let t26207 = t1592 * t551 * t1632 * t8057;
    let t26211 = t2196 * t551 * t1632 * t7591;
    let t26215 = -t26175 + t26178 - t26180 - t26183 - 0.1047928639570397803e1 * t26188 - 0.24451668256642615405e1 * t26191 + 0.98781737744032673979e-1 * t26193 + 0.29272321618148349056e-1 * t26196 + 0.65854491829355115984e-1 * t26198 + 0.34930954652346593433e-1 * t26201 + 0.26004665220162805689e0 * t7313 * t6200 - 0.20803732176130244552e1 * t26207 - 0.83214928704520978207e1 * t26211 + 0.52009330440325611378e0 * t6465 * t8119;
    (t26215,)
}
