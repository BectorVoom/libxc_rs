//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1074/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1074<F: Float>(t310: F, t9779: F, t32003: F, t33535: F, t8406: F, t32130: F, t9029: F, t8397: F, t9054: F, t2146: F, t32135: F, t32143: F, t33778: F, t36432: F, t36436: F, t36439: F, t36447: F, t36452: F, t463: F, t8004: F, t8411: F, t8441: F, t9003: F, t9789: F) -> (F,) {
    let t40844 = t310 * t9779;
    let t40849 = t32003 * t33535 * t8406;
    let t40852 = t32130 * t33535 * t9029;
    let t40858 = t8397 * t9054;
    let t40860 = -0.52041769129231196772e1 * t2146 * t8004 * t9789 * t463 + 0.65854491829355115987e0 * t40844 - t36432 - t36436 + t36439 - t36447 - 0.17347256376410398924e1 * t33778 * t8441 + 0.34694512752820797848e1 * t40849 - 0.34694512752820797848e1 * t40852 + 0.65854491829355115987e0 * t32135 - t36452 + 0.34694512752820797848e1 * t32143 - 0.52041769129231196772e1 * t9003 * t8411 + 0.34694512752820797848e1 * t40858;
    (t40860,)
}
