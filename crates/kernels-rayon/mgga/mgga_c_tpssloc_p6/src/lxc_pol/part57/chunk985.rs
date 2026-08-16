//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 985/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk985(t1880: f64, t8547: f64, t98133: f64, t28263: f64, t31366: f64, t22986: f64, t23270: f64, t31337: f64, t5544: f64, t126413: f64, t31332: f64, t1888: f64, t33457: f64, t86873: f64) -> (f64, f64, f64, f64, f64) {
    let t127798 = t1880 * t98133 * t8547;
    let t127803 = t1880 * t31366 * t28263;
    let t127814 = t22986 * t23270 * t31337 * t5544;
    let t127818 = t22986 * t23270 * t31332 * t126413;
    let t127829 = t1888 * t86873 * t33457;
    (t127798, t127803, t127814, t127818, t127829)
}
