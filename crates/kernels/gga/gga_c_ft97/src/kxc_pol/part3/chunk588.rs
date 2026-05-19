//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 588/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk588<F: Float>(t1073: F, t2281: F, t637: F, t2289: F, t3042: F, t4456: F, t4460: F, t4464: F, t4680: F, t4683: F, t639: F, t2251: F, t2265: F, t3611: F, t3633: F, t4857: F, t4861: F, t4865: F, t4869: F, t631: F) -> (F, F, F, F, F) {
    let t4872 = t1073 * t1073;
    let t4874 = t637 * t2281 * t4872;
    let t4883 = -F::new(0.117377e0) * t4680 + F::new(0.234754e0) * t4683 + t2289 + F::cast_from(0.9628722222222222222e-1_f64) * t3042 - F::cast_from(0.9628722222222222222e-1_f64) * t4456 + F::cast_from(0.28886166666666666666e0_f64) * t4460 - F::cast_from(0.14443083333333333333e0_f64) * t4464;
    let t4885 = t637 * t639 * t4883;
    let t4888 = -t2251 - F::new(2.0) / F::new(9.0) * t3611 - F::new(2.0) / F::new(3.0) * t3633 + t631 * t4857 / F::new(18.0) - F::new(2.0) / F::new(3.0) * t2265 * t4861 - t631 * t4865 / F::new(3.0) + t631 * t4869 / F::new(6.0) - F::new(3.0) / F::new(2.0) * t631 * t4874 + t631 * t4885 / F::new(2.0);
    (t4872, t4874, t4883, t4885, t4888)
}
