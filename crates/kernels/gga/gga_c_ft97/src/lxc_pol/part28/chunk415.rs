//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 415/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk415<F: Float>(t79: F, t1300: F, t1603: F, t1669: F, t5538: F, t5569: F, t5598: F, t5610: F, t5611: F, t6428: F, t6431: F, t6434: F, t6438: F, t6442: F, t6446: F, t6450: F) -> F {
    let t80 = F::new(0.1e-59) < t79;
    let t6454 = piecewise3::<f64>(t80, -F::new(0.23254900946437792e-1) * t1603 * t6428 - F::new(2.0) * t1669 * t6431 + F::new(0.25845121844514357744e-4) * t5538 * t6434 + F::new(0.22227677429409423704e-2) * t1300 * t6438 + F::new(0.22270151833971792333e-3) * t5569 * t6442 + F::new(0.38306165027777777778e-1) * t5598 * t6446 - t5610 - F::new(0.6384360837962962963e-2) * t5611 * t6450, F::new(0.0));
    t6454
}
