//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 912/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk912<F: Float>(t45472: F, t2365: F, t35550: F, t7630: F, t13559: F, t13662: F, t13668: F, t13669: F, t13672: F, t13673: F, t13706: F, t13709: F, t13710: F, t1589: F, t1628: F, t2049: F, t2194: F, t2197: F, t313: F, t314: F, t317: F, t44866: F, t44874: F, t45457: F, t45458: F, t45459: F, t45464: F, t45469: F, t531: F, t568: F, t769: F, t784: F, t797: F, t808: F, t813: F, t833: F, t836: F) -> F {
    let t45473 = F::cast_from(0.42603251059911944084e-1_f64) * t45472;
    let t45475 = t7630 * t2365 * t35550;
    let t45476 = F::cast_from(0.29792074959875355558e-1_f64) * t45475;
    let t45512 = t45457 - t45458 + t45459 - F::cast_from(0.92023022289409799224e1_f64) * t2194 * t13662 - t45464 + t45469 - t45473 - t45476 - F::cast_from(0.35750489951850426669e0_f64) * t2049 * t13706 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t531 * t44874 - F::cast_from(0.23005755572352449806e1_f64) * t2194 * t13710 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t568 * t808 * t44866 + F::cast_from(0.23833659967900284446e0_f64) * t13673 * t784 - F::cast_from(0.30674340763136599741e1_f64) * t813 * t1628 * t13709 - F::cast_from(0.23833659967900284446e0_f64) * t797 * t1589 * t13559 + F::cast_from(0.35750489951850426669e0_f64) * t769 * t13672 * t317 + F::cast_from(0.35750489951850426669e0_f64) * t313 * t314 * t44866 * t317 + F::cast_from(0.30674340763136599741e1_f64) * t833 * t1628 * t13668 + F::cast_from(0.23005755572352449806e1_f64) * t2197 * t13669 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t568 * t836 * t44866;
    t45512
}
