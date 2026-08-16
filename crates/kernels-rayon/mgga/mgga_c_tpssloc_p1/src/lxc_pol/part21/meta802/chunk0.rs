//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2789/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2789(t12940: f64, t58994: f64, t12606: f64, t4194: f64, t4195: f64, t12908: f64, t16713: f64, t12939: f64, t5392: f64, t607: f64, t750: f64, t157: f64, t4196: f64, t46447: f64) -> (f64, f64, f64, f64, f64) {
    let t58996 = 48.0_f64 * t58994 * t12940;
    let t58999 = 24.0_f64 * t4194 * t4195 * t12606;
    let t59001 = 48.0_f64 * t12908 * t16713;
    let t59004 = t12939 * t750 * t5392 * t607;
    let t59005 = 48.0_f64 * t59004;
    let t59008 = 48.0_f64 * t46447 * t157 * t4196;
    (t58996, t58999, t59001, t59005, t59008)
}
