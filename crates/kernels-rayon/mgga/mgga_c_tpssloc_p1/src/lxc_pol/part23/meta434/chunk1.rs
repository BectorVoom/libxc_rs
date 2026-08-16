//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1274/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1274(t18324: f64, t4889: f64, t1174: f64, t135: f64, t22136: f64, t15740: f64, t18371: f64, t1222: f64, t22175: f64, t1734: f64, t6218: f64, t22169: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t72705 = t4889 * t18324;
    let t72708 = t1174 * t135 * t22136;
    let t72727 = t15740 * t18371;
    let t72733 = t22175 * t1222;
    let t72767 = t6218 * t1734;
    let t72798 = t22169 * t1222;
    (t72705, t72708, t72727, t72733, t72767, t72798)
}
