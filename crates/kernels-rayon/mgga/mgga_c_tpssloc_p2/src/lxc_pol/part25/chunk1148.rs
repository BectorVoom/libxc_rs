//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1148/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1148(t1887: f64, t206: f64, t80845: f64, t6605: f64, t9972: f64, t9976: f64, t23133: f64, t2703: f64, t23083: f64, t23089: f64, t23146: f64, t9649: f64) -> (f64, f64, f64, f64, f64) {
    let t81852 = t80845 * t206 * t1887;
    let t81855 = t6605 * t9972 * t9976;
    let t81857 = t23133 * t2703;
    let t81859 = t23083 * t23089;
    let t81861 = t23146 * t9649;
    (t81852, t81855, t81857, t81859, t81861)
}
