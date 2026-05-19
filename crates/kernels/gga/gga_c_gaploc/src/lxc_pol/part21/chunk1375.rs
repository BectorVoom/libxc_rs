//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1375/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1375<F: Float>(t38413: F, t550: F, t1358: F, t1365: F, t30189: F, t30199: F, t30207: F, t32053: F, t32055: F, t32057: F, t32059: F, t32062: F, t32066: F, t32072: F, t32074: F, t32077: F, t32080: F, t32084: F) -> (F, F) {
    let t38447 = t550 * t38413;
    let t38451 = F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t1365 * t38447 + t30189 - t32053 + t32055 - t32057 + t32059 + t32062 - t30199 - t32066 - t32072 - t30207 - t32074 - t32077 - t32080 - t32084;
    (t38447, t38451)
}
