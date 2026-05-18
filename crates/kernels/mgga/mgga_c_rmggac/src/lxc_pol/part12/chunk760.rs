//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 760/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk760<F: Float>(t35618: F, t7490: F, t7552: F, t7558: F, t290: F, t35604: F, t2010: F, t7755: F, t1341: F, t303: F, t638: F, t7310: F) -> (F, F, F, F, F) {
    let t35619 = F::new(0.91462949374725084942e-3) * t35618;
    let t35620 = t7490 * t7552;
    let t35621 = t35620 * t7558;
    let t35622 = F::new(0.13010691197123848594e-3) * t35621;
    let t35623 = t290 * t35604;
    let t35625 = t2010 * t7755 * t35623;
    let t35629 = t638 * t7310 * t303 * t1341;
    (t35619, t35622, t35623, t35625, t35629)
}
