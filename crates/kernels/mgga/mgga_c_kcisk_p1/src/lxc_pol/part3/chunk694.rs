//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 694/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk694<F: Float>(t10809: F, t1773: F, t10487: F, t662: F, t10441: F, t5006: F, t1772: F, t4983: F, t5007: F, t1775: F, t4989: F, t4999: F) -> (F, F, F, F, F) {
    let t10810 = t1773 * t10809;
    let t10812 = t662 * t10487;
    let t10813 = t10812 * t10441;
    let t10814 = t5006 * t10813;
    let t10817 = t4983 * t1772;
    let t10820 = t5007 * t10441;
    let t10821 = t1775 * t10820;
    let t10828 = t4989 * t4999;
    (t10810, t10814, t10817, t10821, t10828)
}
