//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1258/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1258(t1: f64, t10825: f64, t10935: f64, t14390: f64, t24989: f64, t24995: f64, t2672: f64, t297: f64, t313: f64, t3608: f64, t3835: f64, t50745: f64, t50750: f64, t50758: f64, t50761: f64, t50766: f64, t56752: f64, t56756: f64, t56766: f64, t56771: f64, t56775: f64, t8114: f64, t862: f64) -> f64 {
    let t56800 = 0.48295341609937543636e-1_f64 * t3835 * t10935 * t56766 - 0.63777043459628018516e5_f64 * t8114 * t14390 * t56771 + 0.3283935570557285894e5_f64 * t24989 * t313 * t56775 * t2672 * t1 - 0.23456682646837756387e4_f64 * t24995 * t313 * t56775 * t1 * t297 - t862 * t3608 * t56756 / 6.0_f64 + t862 * t3608 * t56752 / 54.0_f64 + 7.0_f64 / 108.0_f64 * t862 * t10825 * t56766 + 0.28345352648723563784e5_f64 * t50745 - 0.48295341609937543636e-1_f64 * t50750 + 0.47242254414539272975e4_f64 * t50758 + 0.21464596271083352727e-1_f64 * t50761 + 0.48295341609937543636e-2_f64 * t50766;
    t56800
}
