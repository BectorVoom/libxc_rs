//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1409/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1409<F: Float>(t1113: F, t136: F, t43800: F, t43804: F, t43759: F, t43766: F, t43768: F, t43770: F, t43773: F, t43777: F, t43833: F, t43835: F, t43837: F, t43839: F, t43842: F, t43845: F) -> (F, F, F) {
    let t43848 = t136 * t1113 * t43800;
    let t43851 = t136 * t1113 * t43804;
    let t43853 = F::cast_from(0.49671e0_f64) * t43759 - F::cast_from(0.8585111111111111111e-1_f64) * t43766 + F::cast_from(0.22076e0_f64) * t43768 - F::cast_from(0.132456e1_f64) * t43770 + F::cast_from(0.22076e0_f64) * t43773 + t43777 + F::cast_from(0.16504875e0_f64) * t43833 + F::cast_from(0.44152e0_f64) * t43835 - F::cast_from(0.132456e1_f64) * t43837 - F::cast_from(0.22076e0_f64) * t43839 + F::cast_from(0.44152e0_f64) * t43842 - F::cast_from(0.99342e0_f64) * t43845 + F::cast_from(0.198684e1_f64) * t43848 + F::cast_from(0.82785e-1_f64) * t43851;
    (t43848, t43851, t43853)
}
