//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1325/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1325(t1891: f64, t1895: f64, t213: f64, t39041: f64, t1887: f64, t206: f64, t80845: f64, t6605: f64, t9972: f64, t9976: f64, t23133: f64, t2703: f64) -> (f64, f64, f64, f64) {
    let t81849 = t39041 * t1891 * t213 * t1895;
    let t81850 = 0.10173934535723378495e0_f64 * t81849;
    let t81852 = t80845 * t206 * t1887;
    let t81853 = 455.0_f64 / 1296.0_f64 * t81852;
    let t81855 = t6605 * t9972 * t9976;
    let t81857 = t23133 * t2703;
    (t81850, t81853, t81855, t81857)
}
