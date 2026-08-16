//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1199/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1199<F: Float>(t32044: F, t10262: F, t2312: F, t23983: F, t2761: F, t6455: F, t10172: F, t30182: F, t30184: F, t30186: F, t32021: F, t32025: F, t32028: F, t32036: F, t32038: F, t32041: F, t32043: F, t4141: F) -> F {
    let t32045 = F::cast_from(0.11856252764865062333e-2_f64) * t32044;
    let t32046 = t2312 * t10262;
    let t32047 = F::cast_from(0.23712505529730124666e-2_f64) * t32046;
    let t32049 = t23983 * t2761 * t6455;
    let t32050 = F::cast_from(0.23712505529730124666e-2_f64) * t32049;
    let t32051 = -t30182 + t32021 - t32025 + t32028 + F::cast_from(0.31616674039640166222e-2_f64) * t4141 * t10172 + t32036 + t32038 - t32041 + t32043 + t32045 + t32047 - t30184 + t30186 + t32050;
    t32051
}
