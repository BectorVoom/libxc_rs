//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1158/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1158(t31992: f64, t31993: f64, t4816: f64, t32000: f64, t4820: f64, t1065: f64, t1646: f64, t1695: f64, t1042: f64, t120181: f64, t120238: f64, t120244: f64, t120259: f64, t120263: f64, t120288: f64, t120305: f64, t120307: f64, t120321: f64, t120495: f64, t1671: f64, t3092: f64, t32014: f64, t33754: f64, t33768: f64, t33827: f64, t33832: f64, t4781: f64, t4825: f64, t906: f64, t93437: f64) -> f64 {
    let t126483 = t31992 * t31993 * t4816;
    let t126487 = t32000 * t4820;
    let t126494 = t1065 * t1646;
    let t126501 = t1065 * t1695;
    let t126508 = -0.18822977838986977999e-3_f64 * t32014 * t3092 * t33768 * t906 - 0.37645955677973955999e-3_f64 * t120321 * t3092 * t33754 * t906 + 0.66110807482757352571e-3_f64 * t120288 * t33832 - 0.82638509353446690713e-4_f64 * t126483 - 0.19833242244827205771e-2_f64 * t120181 * t33827 - 0.24791552806034007213e-3_f64 * t126487 - 0.24791552806034007214e-3_f64 * t120244 + 0.56468933516960933998e-3_f64 * t120305 * t120307 * t4781 * t93437 - 0.3718732920905101082e-3_f64 * t120238 * t1042 * t126494 * t906 + 0.3718732920905101082e-3_f64 * t120259 * t4825 + 0.24791552806034007213e-3_f64 * t120263 * t1042 * t126501 * t906 - 0.3718732920905101082e-3_f64 * t120495 * t1671;
    t126508
}
