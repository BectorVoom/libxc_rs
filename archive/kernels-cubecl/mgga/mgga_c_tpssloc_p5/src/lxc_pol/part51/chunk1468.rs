//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1468/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1468<F: Float>(t120800: F, t120803: F, t122776: F, t122780: F, t122784: F, t122786: F, t122788: F, t122790: F, t122794: F, t2039: F, t23877: F, t31795: F, t4072: F, t7801: F, t7956: F, t83980: F, t86656: F) -> F {
    let t122797 = t122776 + F::cast_from(0.135e2_f64) * t31795 * t4072 + t122780 + F::cast_from(27.0_f64) * t83980 * t7956 + t122784 + t122786 + t122788 + t122790 + F::cast_from(0.135e2_f64) * t23877 * t7801 + t120800 + t120803 + t122794 + F::cast_from(0.135e2_f64) * t86656 * t2039;
    t122797
}
