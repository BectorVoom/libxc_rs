//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1945/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1945(t16918: f64, t23146: f64, t16898: f64, t4191: f64, t87199: f64, t4240: f64, t232: f64, t58569: f64, t6605: f64, t815: f64, t2628: f64, t5585: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98847 = t23146 * t16918;
    let t98849 = t23146 * t16898;
    let t98851 = t87199 * t4191;
    let t98853 = t87199 * t4240;
    let t98858 = t6605 * t815 * t58569 * t232;
    let t98862 = t6605 * t2628 * t5585 * t828;
    (t98847, t98849, t98851, t98853, t98858, t98862)
}
