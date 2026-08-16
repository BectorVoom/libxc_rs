//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1466/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1466(t120865: f64, t120867: f64, t120869: f64, t120871: f64, t122765: f64, t23880: f64, t27273: f64, t27276: f64, t31284: f64, t33195: f64, t577: f64, t7235: f64, t7956: f64, t8508: f64, t86647: f64, t96351: f64) -> f64 {
    let t122774 = 27.0_f64 * t86647 * t7235 + t120865 + t120867 + t31284 + t8508 + t120869 + t120871 + 0.45e1_f64 * t122765 * t577 + 27.0_f64 * t96351 * t7956 + 27.0_f64 * t23880 * t27273 + 27.0_f64 * t23880 * t27276 + t33195;
    t122774
}
