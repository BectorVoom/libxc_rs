//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2032/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2032(t1891: f64, t1895: f64, t213: f64, t39041: f64, t1887: f64, t206: f64, t80845: f64, t23133: f64, t2703: f64, t23083: f64, t23089: f64, t23145: f64, t2617: f64) -> (f64, f64, f64, f64, f64) {
    let t81849 = t39041 * t1891 * t213 * t1895;
    let t81850 = 0.10173934535723378495e0_f64 * t81849;
    let t81852 = t80845 * t206 * t1887;
    let t81853 = 455.0_f64 / 1296.0_f64 * t81852;
    let t81857 = t23133 * t2703;
    let t81859 = t23083 * t23089;
    let t81865 = t2617 * t23145;
    (t81850, t81853, t81857, t81859, t81865)
}
