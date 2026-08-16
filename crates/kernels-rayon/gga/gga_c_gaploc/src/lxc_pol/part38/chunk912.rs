//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 912/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk912(t45472: f64, t2365: f64, t35550: f64, t7630: f64, t13559: f64, t13662: f64, t13668: f64, t13669: f64, t13672: f64, t13673: f64, t13706: f64, t13709: f64, t13710: f64, t1589: f64, t1628: f64, t2049: f64, t2194: f64, t2197: f64, t313: f64, t314: f64, t317: f64, t44866: f64, t44874: f64, t45457: f64, t45458: f64, t45459: f64, t45464: f64, t45469: f64, t531: f64, t568: f64, t769: f64, t784: f64, t797: f64, t808: f64, t813: f64, t833: f64, t836: f64) -> f64 {
    let t45473 = 0.42603251059911944084e-1_f64 * t45472;
    let t45475 = t7630 * t2365 * t35550;
    let t45476 = 0.29792074959875355558e-1_f64 * t45475;
    let t45512 = t45457 - t45458 + t45459 - 0.92023022289409799224e1_f64 * t2194 * t13662 - t45464 + t45469 - t45473 - t45476 - 0.35750489951850426669e0_f64 * t2049 * t13706 - 0.35750489951850426669e0_f64 * t797 * t531 * t44874 - 0.23005755572352449806e1_f64 * t2194 * t13710 - 0.23005755572352449806e1_f64 * t813 * t568 * t808 * t44866 + 0.23833659967900284446e0_f64 * t13673 * t784 - 0.30674340763136599741e1_f64 * t813 * t1628 * t13709 - 0.23833659967900284446e0_f64 * t797 * t1589 * t13559 + 0.35750489951850426669e0_f64 * t769 * t13672 * t317 + 0.35750489951850426669e0_f64 * t313 * t314 * t44866 * t317 + 0.30674340763136599741e1_f64 * t833 * t1628 * t13668 + 0.23005755572352449806e1_f64 * t2197 * t13669 + 0.23005755572352449806e1_f64 * t833 * t568 * t836 * t44866;
    t45512
}
