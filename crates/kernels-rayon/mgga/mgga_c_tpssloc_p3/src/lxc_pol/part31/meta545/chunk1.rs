//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1770/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1770(t59: f64, t9971: f64, t23040: f64, t2617: f64, t23061: f64, t6604: f64, t1891: f64, t1895: f64, t213: f64, t39041: f64, t1887: f64, t206: f64, t80845: f64) -> (f64, f64, f64, f64, f64) {
    let t81816 = t9971 * t59;
    let t81824 = t2617 * t23040;
    let t81835 = t23061 * t6604;
    let t81849 = t39041 * t1891 * t213 * t1895;
    let t81852 = t80845 * t206 * t1887;
    (t81816, t81824, t81835, t81849, t81852)
}
