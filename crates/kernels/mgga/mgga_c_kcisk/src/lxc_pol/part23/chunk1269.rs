//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1269/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1269<F: Float>(t109654: F, t9516: F, t13900: F, t9536: F, t9538: F, t32371: F, t9532: F, t32395: F, t32198: F, t3739: F, t13955: F, t9463: F, t42942: F, t79: F, t1299: F, t1414: F) -> (F, F, F, F, F, F, F, F) {
    let t109669 = t9516 * t109654;
    let t109683 = t9536 * t13900 * t9538;
    let t109690 = t32371 * t9532;
    let t109697 = t32395 * t9532;
    let t109699 = t3739 * t32198;
    let t109701 = t13955 * t9463;
    let t109703 = t42942 * t79;
    let t109717 = t1414 * t1299;
    (t109669, t109683, t109690, t109697, t109699, t109701, t109703, t109717)
}
