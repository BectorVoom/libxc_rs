//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1073/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1073<F: Float>(t1882: F, t35629: F, t35586: F, t110751: F, t11593: F, t13885: F, t14127: F, t141815: F, t141868: F, t141882: F, t149753: F, t1901: F, t242: F, t24793: F, t2606: F, t27924: F, t28294: F, t28299: F, t28300: F, t28308: F, t33754: F, t33759: F, t3746: F, t3837: F, t3842: F, t3859: F, t3864: F, t41408: F, t446: F, t6917: F, t7546: F, t97701: F) -> (F, F, F) {
    let t151633 = t1882 * t35629;
    let t151639 = t1882 * t35586;
    let t151700 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t13885 * t33754 * t3837 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t14127 * t141882 * t3842 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t242 * t149753 + t141815 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t24793 * t28308 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t97701 * t6917 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11593 * t2606 * t33759 * t3746 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t110751 * t28294 + F::cast_from(2.0_f64) * t1901 * t14127 * t141868 * t3859 + F::cast_from(8.0_f64) * t1901 * t28299 * t41408 * t7546 * t3864 - F::cast_from(4.0_f64) * t1901 * t28299 * t28300 * t27924;
    (t151633, t151639, t151700)
}
