//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 959/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk959<F: Float>(t49921: F, t6508: F, t1063: F, t1064: F, t11977: F, t1358: F, t14334: F, t2268: F, t2343: F, t2787: F, t2792: F, t2854: F, t38271: F, t38276: F, t44513: F, t44515: F, t44516: F, t44518: F, t44521: F, t44524: F, t44527: F, t44529: F, t448: F, t46965: F, t47003: F, t49917: F, t6320: F, t6507: F, t8195: F, t993: F) -> (F, F) {
    let t49922 = t6508 * t49921;
    let t49942 = -F::new(0.1707300398140568976e0) * t2268 * t46965 * t993 - F::new(0.1707300398140568976e0) * t2268 * t11977 * t2792 - F::new(0.28455006635676149599e-1) * t1063 * t14334 * t448 + F::new(0.28455006635676149599e-1) * t1063 * t1064 * t49917 - F::new(0.12646669615856066489e-1) * t1358 * t6507 * t49922 + F::new(0.6829201592562275904e0) * t2268 * t2343 * t2787 * t38276 - F::new(0.3414600796281137952e0) * t2268 * t6320 * t2854 * t38276 - F::new(0.3983700928994660944e0) * t2268 * t11977 * t8195 - t44513 + t44515 - F::new(0.63233348079280332443e-2) * t47003 - t44516 - t44518 + t44521 - F::new(0.1138200265427045984e0) * t1063 * t2343 * t2787 * t38271 + t44524 - t44527 + t44529;
    (t49922, t49942)
}
