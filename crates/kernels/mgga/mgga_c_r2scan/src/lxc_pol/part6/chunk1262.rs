//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1262/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1262<F: Float>(t2816: F, t6887: F, t10687: F, t2266: F, t9577: F, t19721: F, t2867: F, t18986: F, t406: F, t7030: F, t18991: F, t2526: F, t6599: F, t6890: F, t18984: F, t18990: F, t18995: F, t23730: F) -> (F, F, F, F, F, F, F, F) {
    let t23731 = t6887 * t2816;
    let t23732 = 0.7089e1 * t23731;
    let t23735 = 9.0 * t2266 * t10687 * t9577;
    let t23738 = 9.0 * t2266 * t2867 * t19721;
    let t23739 = 0.65061487801810439052e-1 * t18986;
    let t23740 = t406 * t7030;
    let t23741 = 12.0 * t23740;
    let t23742 = 36.0 * t18991;
    let t23745 = 9.0 * t2266 * t6599 * t2526;
    let t23748 = 9.0 * t2266 * t6890 * t2526;
    let t23749 = t23730 - t23732 + t23735 + t23738 - t18984 + t23739 + t18990 - t23741 + t23742 - t18995 - t23745 + t23748;
    (t23735, t23738, t23739, t23741, t23742, t23745, t23748, t23749)
}
