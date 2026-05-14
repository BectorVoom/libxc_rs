//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1248/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1248<F: Float>(t34943: F, t10438: F, t1391: F, t587: F, t31160: F, t31163: F, t31166: F, t31169: F, t31172: F, t31175: F, t31178: F, t34928: F, t34931: F, t34935: F, t34937: F, t34939: F, t34941: F) -> (F,) {
    let t34944 = 0.51123901271894332902e0 * t34943;
    let t34946 = t587 * t1391 * t10438;
    let t34947 = 0.2698205900461089792e0 * t34946;
    let t34948 = t31160 - t31163 - t31166 - t31169 - t31172 - t31175 + t31178 - t34928 + t34931 + t34935 - t34937 + t34939 - t34941 + t34944 - t34947;
    (t34948,)
}
