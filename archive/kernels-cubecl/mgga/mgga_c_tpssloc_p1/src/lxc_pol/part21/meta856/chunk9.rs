//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3105/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3105<F: Float>(t43748: F, t63332: F, t63334: F, t63336: F, t63886: F, t63888: F, t63891: F, t63893: F, t63896: F, t63899: F, t63903: F, t63906: F, t63909: F, t63911: F, t63914: F) -> F {
    let t64342 = -F::cast_from(0.89459259259259259257e-1_f64) * t63332 + F::cast_from(0.13418888888888888889e0_f64) * t63334 - F::cast_from(0.20128333333333333334e0_f64) * t63336 - F::cast_from(0.11038e0_f64) * t63886 - F::cast_from(0.30661111111111111112e-1_f64) * t63888 - F::cast_from(0.5519e-1_f64) * t63891 + F::cast_from(0.18396666666666666667e0_f64) * t63893 + F::cast_from(0.33114e0_f64) * t63896 + F::cast_from(0.14717333333333333333e0_f64) * t63899 - F::cast_from(0.8945925925925925926e-1_f64) * t43748 + F::cast_from(0.33114e0_f64) * t63903 + F::cast_from(0.16557e0_f64) * t63906 + F::cast_from(0.49671e0_f64) * t63909 + F::cast_from(0.91983333333333333334e-1_f64) * t63911 + F::cast_from(0.36793333333333333333e-1_f64) * t63914;
    t64342
}
