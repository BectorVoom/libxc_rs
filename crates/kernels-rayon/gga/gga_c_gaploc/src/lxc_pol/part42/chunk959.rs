//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 959/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk959(t49921: f64, t6508: f64, t1063: f64, t1064: f64, t11977: f64, t1358: f64, t14334: f64, t2268: f64, t2343: f64, t2787: f64, t2792: f64, t2854: f64, t38271: f64, t38276: f64, t44513: f64, t44515: f64, t44516: f64, t44518: f64, t44521: f64, t44524: f64, t44527: f64, t44529: f64, t448: f64, t46965: f64, t47003: f64, t49917: f64, t6320: f64, t6507: f64, t8195: f64, t993: f64) -> (f64, f64) {
    let t49922 = t6508 * t49921;
    let t49942 = -0.1707300398140568976e0_f64 * t2268 * t46965 * t993 - 0.1707300398140568976e0_f64 * t2268 * t11977 * t2792 - 0.28455006635676149599e-1_f64 * t1063 * t14334 * t448 + 0.28455006635676149599e-1_f64 * t1063 * t1064 * t49917 - 0.12646669615856066489e-1_f64 * t1358 * t6507 * t49922 + 0.6829201592562275904e0_f64 * t2268 * t2343 * t2787 * t38276 - 0.3414600796281137952e0_f64 * t2268 * t6320 * t2854 * t38276 - 0.3983700928994660944e0_f64 * t2268 * t11977 * t8195 - t44513 + t44515 - 0.63233348079280332443e-2_f64 * t47003 - t44516 - t44518 + t44521 - 0.1138200265427045984e0_f64 * t1063 * t2343 * t2787 * t38271 + t44524 - t44527 + t44529;
    (t49922, t49942)
}
