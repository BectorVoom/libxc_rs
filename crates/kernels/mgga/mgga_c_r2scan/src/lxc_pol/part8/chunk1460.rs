//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1460/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1460<F: Float>(t19720: F, t32980: F, t32982: F, t32984: F, t32985: F, t32986: F, t32987: F, t34858: F, t34861: F, t34866: F, t34870: F, t19611: F, t19614: F, t19620: F, t19624: F, t19628: F, t19646: F, t19649: F, t19687: F, t19728: F, t23951: F, t34876: F) -> (F, F) {
    let t35268 = t34858 + t34861 + t32980 + t34866 + t34870 + t32982 + t32984 - t32985 - t32986 - t32987 + t19720;
    let t35271 = t19611 + t19614 - t19620 + t19624 - t19628 - t19646 - t19649 - t19728 + t34876 + t19687 + t23951;
    (t35268, t35271)
}
