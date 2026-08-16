//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1274/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1274<F: Float>(t18324: F, t4889: F, t1174: F, t135: F, t22136: F, t15740: F, t18371: F, t1222: F, t22175: F, t1734: F, t6218: F, t22169: F) -> (F, F, F, F, F, F) {
    let t72705 = t4889 * t18324;
    let t72708 = t1174 * t135 * t22136;
    let t72727 = t15740 * t18371;
    let t72733 = t22175 * t1222;
    let t72767 = t6218 * t1734;
    let t72798 = t22169 * t1222;
    (t72705, t72708, t72727, t72733, t72767, t72798)
}
