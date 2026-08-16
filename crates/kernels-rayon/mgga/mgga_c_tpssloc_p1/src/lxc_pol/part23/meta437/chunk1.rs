//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1280/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1280(t1174: f64, t15281: f64, t22051: f64, t11539: f64, t22055: f64, t18454: f64, t4889: f64, t22059: f64, t3431: f64, t18529: f64, t135: f64, t22034: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73290 = t1174 * t15281 * t22051;
    let t73307 = t1174 * t11539 * t22055;
    let t73314 = t4889 * t18454;
    let t73330 = t1174 * t3431 * t22059;
    let t73386 = t4889 * t18529;
    let t73389 = t1174 * t135 * t22034;
    (t73290, t73307, t73314, t73330, t73386, t73389)
}
