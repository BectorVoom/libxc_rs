//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 884/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk884<F: Float>(t263: F, t3821: F, t6008: F, t193: F, t6753: F, t681: F, t1403: F, t6930: F, t766: F, t1173: F, t713: F, t3977: F, t6187: F, t6940: F, t2568: F, t1449: F, t3972: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t27963 = t263 * t3821;
    let t27964 = t6008 * t27963;
    let t27965 = t193 * t27964;
    let t27968 = t681 * t6753;
    let t27969 = t1403 * t27968;
    let t27971 = t6930 * t766;
    let t27974 = t1173 * t713;
    let t27975 = t6008 * t27974;
    let t27976 = t193 * t27975;
    let t27981 = t3977 * t6187;
    let t27983 = t6940 * t766;
    let t27984 = t2568 * t27983;
    let t27986 = t1449 * t3972;
    (t27963, t27964, t27965, t27968, t27969, t27971, t27974, t27975, t27976, t27981, t27983, t27984, t27986)
}
