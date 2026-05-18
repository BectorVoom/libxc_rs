//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 747/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk747<F: Float>(t1775: F, t3151: F, t3146: F, t10998: F, t3134: F, t11003: F, t10994: F, t1787: F, t3131: F, t11050: F, t11046: F, t1555: F, t26: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11732 = F::new(4.0) / F::new(3.0) * t1775 * t3151;
    let t11734 = F::new(2.0) / F::new(9.0) * t1775 * t3146;
    let t11735 = t3134 * t10998;
    let t11738 = t3134 * t11003;
    let t11741 = t1787 * t10994;
    let t11745 = F::new(2.0) / F::new(9.0) * t1775 * t3131;
    let t11746 = t3134 * t11050;
    let t11749 = t1787 * t11046;
    let t11755 = t26 * t1555;
    (t11732, t11734, t11735, t11738, t11741, t11745, t11746, t11749, t11755)
}
