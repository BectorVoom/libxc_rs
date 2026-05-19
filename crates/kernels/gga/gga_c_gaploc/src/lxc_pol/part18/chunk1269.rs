//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1269/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1269<F: Float>(t28099: F, t11019: F, t1445: F, t1998: F, t32190: F, t32875: F, t32878: F, t32881: F, t32884: F, t32886: F, t32888: F, t32892: F, t32896: F, t32900: F, t32902: F, t32904: F, t32907: F, t32910: F, t4614: F, t701: F) -> F {
    let t32911 = F::cast_from(0.15976219147466979032e-1_f64) * t28099;
    let t32919 = -t32875 - t32878 - t32881 + t32884 - t32886 + t32888 + t32892 + t32896 + t32900 + t32902 + t32904 - t32907 + t32910 - t32911 - F::cast_from(0.46011511144704899612e1_f64) * t1998 * t1445 * t32190 * t701 - F::cast_from(0.61348681526273199482e1_f64) * t1998 * t4614 * t11019;
    t32919
}
