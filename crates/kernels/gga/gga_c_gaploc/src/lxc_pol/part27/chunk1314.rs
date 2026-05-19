//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1314/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1314<F: Float>(t204: F, t34378: F, t587: F, t10421: F, t21417: F, t10311: F, t4379: F, t30404: F, t10314: F, t20800: F, t6963: F, t18535: F, t19: F, t584: F, t60: F) -> (F, F, F, F, F, F) {
    let t34381 = F::cast_from(0.18404604457881959845e2_f64) * t587 * t204 * t34378;
    let t34382 = t10421 * t21417;
    let t34383 = F::cast_from(0.59584149919750711116e-1_f64) * t34382;
    let t34385 = t4379 * t10311;
    let t34386 = F::cast_from(0.29792074959875355558e-1_f64) * t34385;
    let t34394 = F::cast_from(0.15976219147466979032e-1_f64) * t30404;
    let t34397 = F::cast_from(0.95334639871601137784e0_f64) * t6963 * t20800 * t10314;
    let t34400 = t584 * t18535 * t19 * t60;
    (t34381, t34383, t34386, t34394, t34397, t34400)
}
