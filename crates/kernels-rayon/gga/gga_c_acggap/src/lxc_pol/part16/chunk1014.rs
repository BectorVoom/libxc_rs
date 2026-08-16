//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1014/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1014(t35794: f64, t4680: f64, t7426: f64, t8605: f64, t30468: f64, t4916: f64, t1588: f64, t7614: f64, t1988: f64, t8855: f64, t7799: f64, t8859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35795 = 0.47172138434406228102e-2_f64 * t35794;
    let t35797 = t7426 * t4680 * t8605;
    let t35798 = 0.42874018118069736972e-3_f64 * t35797;
    let t35799 = t30468 * t4916;
    let t35800 = 0.34299214494455789578e-2_f64 * t35799;
    let t35814 = t7614 * t1588;
    let t35816 = t1988 * t8855;
    let t35817 = 0.21437009059034868486e-3_f64 * t35816;
    let t35818 = t7799 * t8859;
    (t35795, t35798, t35800, t35814, t35817, t35818)
}
