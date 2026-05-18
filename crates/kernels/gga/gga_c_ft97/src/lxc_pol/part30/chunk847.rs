//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 847/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk847<F: Float>(t28128: F, t6930: F, t14127: F, t24789: F, t6848: F, t1091: F, t33754: F, t2606: F, t7502: F, t10007: F, t265: F, t35516: F, t729: F) -> (F, F, F, F, F, F, F, F) {
    let t35613 = t28128 * t6930;
    let t35614 = t14127 * t35613;
    let t35617 = t24789 * t6848;
    let t35620 = t33754 * t1091;
    let t35621 = t2606 * t35620;
    let t35624 = t7502 * t1091;
    let t35625 = t10007 * t35624;
    let t35629 = t729 * t265 * t35516;
    (t35613, t35614, t35617, t35620, t35621, t35624, t35625, t35629)
}
