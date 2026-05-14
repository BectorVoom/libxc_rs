//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 807/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk807<F: Float>(t1476: F, t2739: F, t2781: F, t1486: F, t193: F, t6260: F, t824: F, t6323: F, t681: F, t2789: F, t852: F, t6308: F, t856: F, t10570: F, t24949: F, t10631: F, t91: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25000 = t1476 * t2739;
    let t25001 = t2781 * t25000;
    let t25003 = t1486 * t193 * t25001;
    let t25004 = t6260 * t824;
    let t25005 = t2781 * t25004;
    let t25007 = t1486 * t193 * t25005;
    let t25010 = t1486 * t681 * t6323;
    let t25012 = t1476 * t2789;
    let t25013 = t852 * t25012;
    let t25015 = t6308 * t193 * t25013;
    let t25017 = t6260 * t856;
    let t25018 = t852 * t25017;
    let t25020 = t6308 * t193 * t25018;
    let t25022 = t10570 * t24949;
    let t25024 = t1486 * t193 * t25022;
    let t25026 = t91 * t10631;
    (t25000, t25001, t25003, t25004, t25005, t25007, t25010, t25013, t25015, t25018, t25020, t25022, t25024, t25026)
}
