//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 793/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk793<F: Float>(t11795: F, t11797: F, t11800: F, t11803: F, t11806: F, t11811: F, t11813: F, t11815: F, t11817: F, t11820: F, t201: F, t219: F, t11787: F, t2801: F, t779: F, t229: F, t2827: F) -> (F, F, F) {
    let t11825 = 1.0 * t201 * (-0.21099166666666666667e1 * t11795 + 0.202552e2 * t11797 - 0.75019259259259259258e1 * t11800 + 0.6564185185185185185e1 * t11803 + 0.31003950617283950618e1 * t11806 + 0.68258333333333333335e-1 * t11811 - 0.10921333333333333333e1 * t11813 + 0.12134814814814814815e1 * t11815 + 0.10617962962962962963e1 * t11817 + 0.13388493827160493828e1 * t11820) * t219;
    let t11828 = 0.57895126195293126241e3 * t2801 * t11787 * t779;
    let t11829 = t229 * t2827;
    (t11825, t11828, t11829)
}
