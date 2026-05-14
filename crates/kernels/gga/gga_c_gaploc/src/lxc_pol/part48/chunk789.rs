//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 789/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk789<F: Float>(t45475: F, t13559: F, t13662: F, t13668: F, t13669: F, t13672: F, t13673: F, t13706: F, t13709: F, t13710: F, t1589: F, t1628: F, t2049: F, t2194: F, t2197: F, t313: F, t314: F, t317: F, t44866: F, t44874: F, t45457: F, t45458: F, t45459: F, t45464: F, t45469: F, t45473: F, t531: F, t568: F, t769: F, t784: F, t797: F, t808: F, t813: F, t833: F, t836: F) -> (F,) {
    let t45476 = 0.29792074959875355558e-1 * t45475;
    let t45512 = t45457 - t45458 + t45459 - 0.92023022289409799224e1 * t2194 * t13662 - t45464 + t45469 - t45473 - t45476 - 0.35750489951850426669e0 * t2049 * t13706 - 0.35750489951850426669e0 * t797 * t531 * t44874 - 0.23005755572352449806e1 * t2194 * t13710 - 0.23005755572352449806e1 * t813 * t568 * t808 * t44866 + 0.23833659967900284446e0 * t13673 * t784 - 0.30674340763136599741e1 * t813 * t1628 * t13709 - 0.23833659967900284446e0 * t797 * t1589 * t13559 + 0.35750489951850426669e0 * t769 * t13672 * t317 + 0.35750489951850426669e0 * t313 * t314 * t44866 * t317 + 0.30674340763136599741e1 * t833 * t1628 * t13668 + 0.23005755572352449806e1 * t2197 * t13669 + 0.23005755572352449806e1 * t833 * t568 * t836 * t44866;
    (t45512,)
}
