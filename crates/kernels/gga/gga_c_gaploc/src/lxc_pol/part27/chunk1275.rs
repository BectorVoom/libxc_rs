//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1275/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1275<F: Float>(t32948: F, t6066: F, t6111: F, t10914: F, t10915: F, t10847: F, t2615: F, t818: F, t33557: F, t7584: F, t7585: F, t33155: F) -> (F, F, F, F, F) {
    let t33610 = F::new(0.85801175884441024006e1) * t6111 * t6066 * t32948;
    let t33613 = F::new(0.42900587942220512002e1) * t10914 * t10915 * t32948;
    let t33616 = F::new(0.12269736305254639897e2) * t2615 * t818 * t10847;
    let t33619 = F::new(0.23005755572352449806e2) * t7584 * t7585 * t33557;
    let t33624 = F::new(0.11502877786176224903e2) * t7584 * t7585 * t33155;
    (t33610, t33613, t33616, t33619, t33624)
}
