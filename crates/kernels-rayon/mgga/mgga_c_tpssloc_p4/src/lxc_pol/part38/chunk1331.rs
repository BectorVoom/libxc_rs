//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1331/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1331(t110736: f64, t110778: f64, t110826: f64, t110870: f64, t110671: f64, t110684: f64, t12524: f64, t12813: f64, t1458: f64, t16521: f64, t16524: f64, t16538: f64, t16541: f64, t2180: f64, t2363: f64, t29934: f64, t29993: f64, t29996: f64, t30012: f64, t30180: f64, t30231: f64, t30253: f64, t30258: f64, t3941: f64, t4072: f64, t55341: f64, t55353: f64, t55571: f64, t577: f64, t671: f64, t8143: f64, t8166: f64, t8230: f64, t8251: f64) -> (f64, f64) {
    let t110872 = t110736 + t110778 + t110826 + t110870;
    let t110877 = 27.0_f64 * t29996 * t16541 + 54.0_f64 * t29996 * t16538 + 54.0_f64 * t12524 * t30258 + 27.0_f64 * t16524 * t30012 + 54.0_f64 * t12524 * t30253 + 27.0_f64 * t3941 * t29934 * t1458 + 27.0_f64 * t16521 * t8143 + 27.0_f64 * t110671 * t1458 + 27.0_f64 * t29993 * t4072 + 54.0_f64 * t3941 * t30180 * t671 + 27.0_f64 * t3941 * t8230 * t2363 + 0.135e2_f64 * t30231 * t2363 + 27.0_f64 * t110684 * t671 + 27.0_f64 * t3941 * t2180 * t12813 + 27.0_f64 * t55571 * t8251 + 0.135e2_f64 * t55341 * t2180 + 0.45e1_f64 * t110872 * t577 + 54.0_f64 * t55353 * t8166;
    (t110872, t110877)
}
