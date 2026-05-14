//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1012/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1012<F: Float>(t13287: F, t2297: F, t31195: F, t3169: F, t15386: F, t35340: F, t2288: F, t4210: F, t31057: F, t3176: F, t33953: F, t31443: F, t31482: F, t31484: F, t31487: F, t31489: F, t31492: F, t35668: F, t35670: F, t35673: F, t35674: F, t35676: F, t35679: F, t35683: F, t35686: F, t35691: F) -> (F, F, F) {
    let t35695 = t31195 * t13287 * t2297 * t3169;
    let t35698 = t31195 * t15386 * t35340;
    let t35700 = t2288 * t4210;
    let t35702 = t31057 * t15386 * t35700;
    let t35703 = 0.94344276868812456204e-3 * t35702;
    let t35704 = t33953 * t3176;
    let t35706 = t31443 * t13287 * t35704;
    let t35708 = 0.17149607247227894789e-1 * t35668 + 0.85748036236139473945e-2 * t35670 - t35673 - 0.68598428988911579156e-2 * t35674 - 0.34299214494455789578e-2 * t35676 + t35679 - 0.28582678745379824648e-3 * t31482 - t35683 - 0.25724410870841842183e-2 * t31484 - t35686 + t31487 / 96.0 - 0.4584375e-1 * t31489 - 0.916875e-1 * t31492 + 0.94344276868812456204e-2 * t35691 - 0.31448092289604152068e-2 * t35695 + 0.47172138434406228102e-2 * t35698 + t35703 + 0.42874018118069736972e-3 * t35706;
    (t35700, t35704, t35708)
}
