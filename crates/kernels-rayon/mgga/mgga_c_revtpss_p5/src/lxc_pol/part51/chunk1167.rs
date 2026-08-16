//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1167/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1167(t1035: f64, t1695: f64, t1042: f64, t1043: f64, t1045: f64, t1078: f64, t1089: f64, t1096: f64, t120275: f64, t120305: f64, t120307: f64, t120313: f64, t120400: f64, t120425: f64, t120507: f64, t120538: f64, t120671: f64, t120708: f64, t120709: f64, t1646: f64, t1669: f64, t247: f64, t27441: f64, t3116: f64, t31891: f64, t31934: f64, t31935: f64, t31959: f64, t33808: f64, t33811: f64, t33812: f64, t33825: f64, t4786: f64, t4866: f64, t4906: f64, t5015: f64, t8502: f64) -> f64 {
    let t126819 = t1035 * t1695;
    let t126828 = 0.56468933516960933998e-3_f64 * t120305 * t120307 * t1646 * t1043 * t1045 - 0.56468933516960933998e-3_f64 * t120313 * t120307 * t4906 + 0.7437465841810202164e-3_f64 * t120275 * t1042 * t33825 * t4786 - 0.11156198762715303246e-2_f64 * t120708 * t1042 * t1669 * t120709 + 0.34694512752820797848e1_f64 * t120400 * t27441 - 0.37645955677973955998e-3_f64 * t120538 + 0.11423947533020470523e1_f64 * t120671 * t33808 + 0.11423947533020470523e1_f64 * t31934 * t31935 * t4866 * t1089 - 0.28234466758480466999e-3_f64 * t8502 * t247 * t3116 * t1078 * t5015 + 0.11423947533020470523e1_f64 * t120425 * t33812 + 0.17347256376410398924e1_f64 * t120507 * t126819 * t1043 * t1089 - 0.34271842599061411569e1_f64 * t31891 * t31959 * t33811 * t1096;
    t126828
}
