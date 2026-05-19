//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1373/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1373<F: Float>(t10311: F, t4379: F, t30404: F, t10314: F, t20800: F, t6963: F, t18535: F, t19: F, t584: F, t60: F, t18540: F, t201: F) -> (F, F, F, F, F) {
    let t34385 = t4379 * t10311;
    let t34386 = F::cast_from(0.29792074959875355558e-1_f64) * t34385;
    let t34394 = F::cast_from(0.15976219147466979032e-1_f64) * t30404;
    let t34397 = F::cast_from(0.95334639871601137784e0_f64) * t6963 * t20800 * t10314;
    let t34400 = t584 * t18535 * t19 * t60;
    let t34401 = t201 * t18540;
    (t34386, t34394, t34397, t34400, t34401)
}
