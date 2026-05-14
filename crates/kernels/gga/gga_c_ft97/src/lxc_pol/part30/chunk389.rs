//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 389/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk389<F: Float>(t6035: F, t6804: F, t3766: F, t6054: F, t1113: F, t231: F, t39: F, t694: F, t5585: F, t3789: F) -> (F, F, F, F, F, F) {
    let t6805 = t6035 * t6804;
    let t6808 = t3766 * t6054;
    let t6809 = t231 * t1113;
    let t6813 = t694 * t39;
    let t6814 = t6813 * t5585;
    let t6815 = t3789 * t6814;
    (t6805, t6808, t6809, t6813, t6814, t6815)
}
