//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1073/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1073(t1882: f64, t35629: f64, t35586: f64, t110751: f64, t11593: f64, t13885: f64, t14127: f64, t141815: f64, t141868: f64, t141882: f64, t149753: f64, t1901: f64, t242: f64, t24793: f64, t2606: f64, t27924: f64, t28294: f64, t28299: f64, t28300: f64, t28308: f64, t33754: f64, t33759: f64, t3746: f64, t3837: f64, t3842: f64, t3859: f64, t3864: f64, t41408: f64, t446: f64, t6917: f64, t7546: f64, t97701: f64) -> (f64, f64, f64) {
    let t151633 = t1882 * t35629;
    let t151639 = t1882 * t35586;
    let t151700 = -2.0_f64 / 3.0_f64 * t1901 * t13885 * t33754 * t3837 - 2.0_f64 / 3.0_f64 * t1901 * t14127 * t141882 * t3842 + 2.0_f64 / 3.0_f64 * t446 * t242 * t149753 + t141815 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t1901 * t24793 * t28308 + 2.0_f64 / 9.0_f64 * t1901 * t97701 * t6917 + 4.0_f64 / 9.0_f64 * t11593 * t2606 * t33759 * t3746 - 4.0_f64 / 3.0_f64 * t1901 * t110751 * t28294 + 2.0_f64 * t1901 * t14127 * t141868 * t3859 + 8.0_f64 * t1901 * t28299 * t41408 * t7546 * t3864 - 4.0_f64 * t1901 * t28299 * t28300 * t27924;
    (t151633, t151639, t151700)
}
