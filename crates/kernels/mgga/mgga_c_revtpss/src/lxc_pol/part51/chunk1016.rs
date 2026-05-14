//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1016/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1016<F: Float>(t246: F, t3148: F, t100743: F, t1042: F, t1043: F, t1045: F, t11703: F, t120238: F, t120259: F, t120275: F, t120276: F, t120306: F, t120307: F, t120318: F, t120322: F, t120376: F, t120412: F, t120429: F, t12046: F, t120460: F, t120466: F, t12077: F, t126448: F, t1669: F, t1671: F, t1695: F, t1982: F, t2852: F, t31992: F, t31993: F, t32013: F, t32014: F, t32015: F, t385: F, t4181: F, t4757: F, t4830: F, t4872: F, t4875: F, t4895: F, t4901: F, t99970: F) -> (F,) {
    let t126572 = t3148 * t246;
    let t126596 = 0.7437465841810202164e-3 * t120275 * t1042 * t1669 * t120276 - 0.56468933516960933998e-3 * t120376 * t32015 * t120322 * t100743 - 0.112937867033921868e-2 * t120466 * t32015 * t120306 * t99970 + 0.16940680055088280199e-2 * t120412 * t32013 * t32015 * t120306 * t4757 - 0.28234466758480466999e-3 * t120429 * t120460 * t1695 * t1043 * t1045 + 0.19833242244827205771e-2 * t120318 * t1671 + 0.56468933516960933998e-3 * t1982 * t12077 * t126572 * t120307 * t4895 - 0.28234466758480466999e-3 * t1982 * t12046 * t126572 * t120307 * t4901 - 0.37187329209051010821e-3 * t120238 * t1042 * t4872 * t126448 + 0.37187329209051010821e-3 * t120259 * t4875 - 0.12395776403017003607e-3 * t31992 * t31993 * t4830 + 0.31371629731644963332e-3 * t32014 * t11703 * t385 * t2852 * t4181;
    (t126596,)
}
