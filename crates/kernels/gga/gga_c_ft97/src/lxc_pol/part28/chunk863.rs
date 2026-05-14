//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 863/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk863<F: Float>(t136457: F, t32809: F, t32795: F, t32796: F, t549: F, t1691: F, t136678: F, t23742: F, t138838: F, t23842: F, t138873: F, t527: F, t2058: F, t5551: F, t133: F, t1995: F) -> (F, F, F, F, F, F, F, F, F) {
    let t138996 = t32809 * t136457;
    let t139009 = t32795 * t32796 * t549;
    let t139046 = t549 * t1691;
    let t139057 = t23742 * t136678;
    let t139065 = t23842 * t138838;
    let t139082 = t527 * t138873;
    let t139086 = t2058 * t5551;
    let t139087 = t133 * t139086;
    let t139098 = t1995 * t138873;
    (t138996, t139009, t139046, t139057, t139065, t139082, t139086, t139087, t139098)
}
