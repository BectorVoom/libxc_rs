//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 923/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk923<F: Float>(t169: F, t4598: F, t766: F, t242: F, t6054: F, t5697: F, t700: F, t1339: F, t1383: F, t1763: F, t47: F, t1696: F, t52: F) -> (F, F, F, F, F, F) {
    let t19001 = F::cast_from(0.2122377718311958218e0_f64) * t169 * t766 * t4598;
    let t19004 = F::cast_from(0.24210827305188264118e1_f64) * t169 * t6054 * t242;
    let t19035 = F::cast_from(0.20752137690161369243e1_f64) * t169 * t5697 * t700;
    let t19044 = F::cast_from(0.84895108732478328721e0_f64) * t169 * t1339 * t1383;
    let t19058 = F::new(1.0) / t47 / t1763;
    let t19071 = F::new(1.0) / t52 / t1696;
    (t19001, t19004, t19035, t19044, t19058, t19071)
}
