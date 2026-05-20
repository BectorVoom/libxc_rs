//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1650/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1650<F: Float>(t43762: F, t43769: F, t43771: F, t43773: F, t43779: F, t43781: F, t43783: F, t43785: F, t43787: F, t43791: F, t43795: F, t43799: F, t43802: F, t43804: F) -> F {
    let t45103 = -F::cast_from(0.12349037037037037037e0_f64) * t43762 - F::cast_from(0.10805407407407407407e0_f64) * t43769 - F::cast_from(0.12349037037037037037e1_f64) * t43771 + F::cast_from(0.55570666666666666668e0_f64) * t43773 + F::cast_from(0.55570666666666666666e0_f64) * t43779 + F::cast_from(0.69463333333333333334e0_f64) * t43781 + F::cast_from(0.13892666666666666667e1_f64) * t43783 - F::cast_from(0.27785333333333333333e0_f64) * t43785 - F::new(0.166712e1) * t43787 - F::new(0.125034e1) * t43791 + F::new(0.250068e1) * t43795 + F::new(0.104195e0) * t43799 + F::cast_from(0.158837625e2_f64) * t43802 - F::new(0.705945e1) * t43804;
    t45103
}
