//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 432/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk432<F: Float>(t140: F, t5579: F, t6608: F, t1355: F, t2043: F, t5785: F, t5802: F, t5813: F, t5829: F, t5837: F, t5838: F, t6450: F, t6593: F, t6597: F, t6605: F) -> F {
    let t141 = F::new(0.1e-59) < t140;
    let t6609 = t5579 * t6608;
    let t6615 = piecewise3::<f64>(t141, F::new(0.45306850413028723348e0) * t5785 * t6593 - F::new(0.22653425206514361674e0) * t2043 * t6597 - F::new(0.45306850413028723348e0) * t5802 * t6593 + F::new(0.22653425206514361674e0) * t1355 * t6597 - F::new(0.10001700163888888889e0) * t5813 * t6605 + F::new(0.10001700163888888889e0) * t5829 * t6609 - t5837 - F::new(0.16669500273148148149e-1) * t5838 * t6450, F::new(0.0));
    t6615
}
