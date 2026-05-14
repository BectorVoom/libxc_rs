//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1223/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1223<F: Float>(t42957: F, t79: F, t12261: F, t2737: F, t9543: F, t9518: F, t32473: F, t9535: F, t9516: F, t13900: F, t9536: F, t9538: F, t13955: F, t9463: F, t42942: F, t2736: F) -> (F, F, F, F, F, F, F, F) {
    let t109645 = t42957 * t79;
    let t109652 = t2737 * t12261 * t9543;
    let t109654 = t12261 * t9518;
    let t109655 = t2737 * t109654;
    let t109664 = t32473 * t9535;
    let t109669 = t9516 * t109654;
    let t109683 = t9536 * t13900 * t9538;
    let t109701 = t13955 * t9463;
    let t109703 = t42942 * t79;
    let t109704 = t109703 * t2736;
    (t109645, t109652, t109655, t109664, t109669, t109683, t109701, t109704)
}
