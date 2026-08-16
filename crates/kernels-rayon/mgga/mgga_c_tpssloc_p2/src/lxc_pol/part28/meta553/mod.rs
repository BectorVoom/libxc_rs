//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta553 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1823;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta553(t2031: f64, t83718: f64, t2240: f64, t240: f64, t33: f64, t6492: f64, t2244: f64, t63: f64, t23993: f64, t6495: f64, t1860: f64, t22489: f64, t7031: f64, t1864: f64, t67: f64, t835: f64, t22534: f64, t7032: f64, t6486: f64, t24165: f64, t532: f64, t80743: f64, t81281: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t84237, t84241, t84242, t84245, t84248, t84270) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1823(t2031, t83718, t2240, t240, t33, t6492, t2244, t63, t23993, t6495, t1860, t22489, t7031);
        let (t84280, t84283, t84285, t84347, t84400, t84423) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1824(t1860, t1864, t67, t835, t22534, t7032, t23993, t6486, t24165, t532, t80743, t81281);
    (t84237, t84241, t84242, t84245, t84248, t84270, t84280, t84283, t84285, t84347, t84400, t84423)
}
