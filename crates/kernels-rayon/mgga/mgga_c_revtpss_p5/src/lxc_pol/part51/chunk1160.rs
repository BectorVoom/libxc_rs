//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1160/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1160(t246: f64, t3148: f64, t100743: f64, t1042: f64, t1043: f64, t1045: f64, t11703: f64, t120238: f64, t120259: f64, t120275: f64, t120276: f64, t120306: f64, t120307: f64, t120318: f64, t120322: f64, t120376: f64, t120412: f64, t120429: f64, t12046: f64, t120460: f64, t120466: f64, t12077: f64, t126448: f64, t1669: f64, t1671: f64, t1695: f64, t1982: f64, t2852: f64, t31992: f64, t31993: f64, t32013: f64, t32014: f64, t32015: f64, t385: f64, t4181: f64, t4757: f64, t4830: f64, t4872: f64, t4875: f64, t4895: f64, t4901: f64, t99970: f64) -> f64 {
    let t126572 = t3148 * t246;
    let t126596 = 0.7437465841810202164e-3_f64 * t120275 * t1042 * t1669 * t120276 - 0.56468933516960933998e-3_f64 * t120376 * t32015 * t120322 * t100743 - 0.112937867033921868e-2_f64 * t120466 * t32015 * t120306 * t99970 + 0.16940680055088280199e-2_f64 * t120412 * t32013 * t32015 * t120306 * t4757 - 0.28234466758480466999e-3_f64 * t120429 * t120460 * t1695 * t1043 * t1045 + 0.19833242244827205771e-2_f64 * t120318 * t1671 + 0.56468933516960933998e-3_f64 * t1982 * t12077 * t126572 * t120307 * t4895 - 0.28234466758480466999e-3_f64 * t1982 * t12046 * t126572 * t120307 * t4901 - 0.37187329209051010821e-3_f64 * t120238 * t1042 * t4872 * t126448 + 0.37187329209051010821e-3_f64 * t120259 * t4875 - 0.12395776403017003607e-3_f64 * t31992 * t31993 * t4830 + 0.31371629731644963332e-3_f64 * t32014 * t11703 * t385 * t2852 * t4181;
    t126596
}
