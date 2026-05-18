//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 854/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk854<F: Float>(t3977: F, t7553: F, t242: F, t1168: F, t7440: F, t2574: F, t762: F, t1424: F, t6947: F, t729: F, t33274: F, t35304: F) -> (F, F, F, F, F, F, F, F) {
    let t35693 = t3977 * t7553;
    let t35694 = t242 * t35693;
    let t35697 = t7440 * t1168;
    let t35699 = t2574 * t762 * t35697;
    let t35703 = t729 * t6947 * t1424;
    let t35706 = t33274 * t1168;
    let t35707 = t242 * t35706;
    let t35710 = t242 * t35304;
    (t35693, t35694, t35697, t35699, t35703, t35706, t35707, t35710)
}
