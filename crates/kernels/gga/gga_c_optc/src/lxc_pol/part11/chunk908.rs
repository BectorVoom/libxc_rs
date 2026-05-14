//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 908/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk908<F: Float>(t1: F, t17855: F, t438: F, t450: F, t16236: F, t8516: F, t894: F, t17352: F, t3235: F, t17344: F, t4387: F, t1121: F, t1133: F, t12098: F, t12106: F, t17700: F, t17705: F, t17710: F, t17714: F, t17720: F, t17724: F, t17729: F, t3132: F, t4369: F, t4386: F, t5298: F, t5302: F, t8913: F, t8921: F, t8960: F) -> (F, F, F, F, F, F, F) {
    let t17857 = t17855 * t1 * t438;
    let t17858 = t450 * t17857;
    let t17863 = t8516 * t16236;
    let t17864 = t894 * t17863;
    let t17869 = t3235 * t17352;
    let t17872 = t4387 * t17344;
    let t17875 = 0.17715845405452227366e4 * t8960 * t17700 + 0.10629507243271336419e5 * t8913 * t17705 - 0.10629507243271336419e5 * t8921 * t17710 + 0.10866451862235947318e-1 * t1133 * t17714 - 0.48295341609937543638e-1 * t4369 * t5302 + 0.18110753103726578864e-2 * t1133 * t17720 + 0.80492236016562572728e-2 * t1133 * t17724 - 0.13735917720689745254e2 * t3132 * t17729 + 0.35500316489081544176e-1 * t1121 * t17858 - 0.28977204965962526182e-1 * t4369 * t5298 - 0.18110753103726578864e-1 * t1133 * t17864 + 0.96590683219875087275e-2 * t12098 - 0.23666877659387696117e-1 * t12106 - 0.10866451862235947318e-1 * t4386 * t17869 + 0.90553765518632894319e-2 * t4386 * t17872;
    (t17857, t17858, t17863, t17864, t17869, t17872, t17875)
}
