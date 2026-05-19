//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1190/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1190<F: Float>(t32044: F, t10262: F, t2312: F, t23983: F, t2761: F, t6455: F, t10167: F, t29874: F, t10269: F, t4141: F, t10196: F, t3833: F) -> (F, F, F, F, F, F) {
    let t32045 = F::cast_from(0.11856252764865062333e-2_f64) * t32044;
    let t32046 = t2312 * t10262;
    let t32047 = F::cast_from(0.23712505529730124666e-2_f64) * t32046;
    let t32049 = t23983 * t2761 * t6455;
    let t32050 = F::cast_from(0.23712505529730124666e-2_f64) * t32049;
    let t32052 = t29874 * t10167;
    let t32053 = F::cast_from(0.71137516589190373998e-2_f64) * t32052;
    let t32055 = F::cast_from(0.63233348079280332441e-2_f64) * t4141 * t10269;
    let t32057 = F::cast_from(0.56910013271352299198e-1_f64) * t3833 * t10196;
    (t32045, t32047, t32050, t32053, t32055, t32057)
}
