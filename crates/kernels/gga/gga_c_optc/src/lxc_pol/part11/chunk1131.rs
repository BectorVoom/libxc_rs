//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1131/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1131<F: Float>(t1: F, t10825: F, t10935: F, t14390: F, t24989: F, t24995: F, t2672: F, t297: F, t313: F, t3608: F, t3835: F, t50745: F, t50750: F, t50758: F, t50761: F, t50766: F, t56752: F, t56756: F, t56766: F, t56771: F, t56775: F, t8114: F, t862: F) -> (F,) {
    let t56800 = 0.48295341609937543636e-1 * t3835 * t10935 * t56766 - 0.63777043459628018516e5 * t8114 * t14390 * t56771 + 0.3283935570557285894e5 * t24989 * t313 * t56775 * t2672 * t1 - 0.23456682646837756387e4 * t24995 * t313 * t56775 * t1 * t297 - t862 * t3608 * t56756 / 6.0 + t862 * t3608 * t56752 / 54.0 + 7.0 / 108.0 * t862 * t10825 * t56766 + 0.28345352648723563784e5 * t50745 - 0.48295341609937543636e-1 * t50750 + 0.47242254414539272975e4 * t50758 + 0.21464596271083352727e-1 * t50761 + 0.48295341609937543636e-2 * t50766;
    (t56800,)
}
