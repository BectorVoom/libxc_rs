//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1129/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1129(t34210: f64, t34384: f64, t3: f64, t1458: f64, t32643: f64, t33184: f64, t33187: f64, t33190: f64, t33192: f64, t33195: f64, t33774: f64, t33776: f64, t33778: f64, t577: f64, t8508: f64) -> (f64, f64, f64) {
    let t34385 = t34210 + t34384;
    let t34386 = t3 * t34385;
    let t34401 = 0.45e1_f64 * t34385 * t577 + 0.135e2_f64 * t32643 * t1458 + 27.0_f64 * t33774 + 54.0_f64 * t33776 + 27.0_f64 * t33778 + t33184 + t33187 + t33190 + t33192 + t33195 + t8508;
    (t34385, t34386, t34401)
}
