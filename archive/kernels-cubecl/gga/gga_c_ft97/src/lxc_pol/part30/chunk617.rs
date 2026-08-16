//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 617/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk617<F: Float>(t1173: F, t713: F, t6008: F, t193: F, t3977: F, t6187: F, t6940: F, t766: F, t2568: F, t1449: F, t3972: F, t10002: F, t6930: F) -> (F, F, F, F, F, F, F, F) {
    let t27974 = t1173 * t713;
    let t27975 = t6008 * t27974;
    let t27976 = t193 * t27975;
    let t27981 = t3977 * t6187;
    let t27983 = t6940 * t766;
    let t27984 = t2568 * t27983;
    let t27986 = t1449 * t3972;
    let t27987 = t2568 * t27986;
    let t27989 = t10002 * t6930;
    (t27974, t27976, t27981, t27983, t27984, t27986, t27987, t27989)
}
