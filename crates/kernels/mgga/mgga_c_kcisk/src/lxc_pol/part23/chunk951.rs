//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 951/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk951<F: Float>(t2075: F, t3539: F, t3587: F, t1175: F, t5601: F, t3544: F, t3619: F, t1364: F, t19136: F, t5907: F, t19132: F, t13129: F, t3559: F, t19114: F, t5895: F, t3521: F, t5900: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19287 = t3539 * t2075 * t3587;
    let t19291 = t3539 * t5601 * t1175;
    let t19295 = t3544 * t2075 * t3619;
    let t19299 = t3544 * t5601 * t1364;
    let t19302 = t5907 * t19136;
    let t19305 = t5907 * t19132;
    let t19311 = t13129 * t2075 * t3559;
    let t19314 = t5895 * t19114;
    let t19318 = 0.13140859333333333334e-2 * t3521 * t5900;
    (t19287, t19291, t19295, t19299, t19302, t19305, t19311, t19314, t19318)
}
