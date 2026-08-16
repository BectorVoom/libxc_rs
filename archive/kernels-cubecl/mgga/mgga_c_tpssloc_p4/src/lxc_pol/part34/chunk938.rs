//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 938/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk938<F: Float>(t11247: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t1107: F) -> (F, F) {
    let t21780 = -t11247 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14702 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t18203 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18219 - t18229 / F::cast_from(3.0_f64) + F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t21760 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t21764 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t21767 + F::cast_from(2.0_f64) * t21771 + F::cast_from(2.0_f64) * t21774 + t21778 / F::cast_from(3.0_f64);
    let t21781 = t1107 * t21780;
    (t21780, t21781)
}
