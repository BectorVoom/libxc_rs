//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 931/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk931(t1096: f64, t373: f64, t372: f64, t371: f64, t1035: f64, t1043: f64, t1089: f64, t1984: f64, t31885: f64, t31888: f64, t31891: f64, t31894: f64, t31897: f64, t31899: f64, t31903: f64, t31905: f64, t31909: f64, t31913: f64, t31916: f64, t31920: f64, t31923: f64, t31928: f64, t31934: f64, t31937: f64, t31940: f64, t31943: f64, t31950: f64, t359: f64, t7174: f64, t8502: f64, t8524: f64, t999: f64) -> (f64, f64, f64) {
    let t31951 = t373 * t1096;
    let t31952 = t372 * t31951;
    let t31953 = t371 * t31952;
    let t31956 = -t31885 - 0.28234466758480466999e-3_f64 * t8502 * t31888 + 0.11423947533020470523e1_f64 * t31891 * t31894 + 0.17135921299530705785e1_f64 * t31897 * t31899 - 0.17135921299530705785e1_f64 * t31903 * t31905 - 0.1859366460452550541e-3_f64 * t31909 * t8524 + 0.56468933516960933998e-3_f64 * t31913 * t31916 - 0.56468933516960933998e-3_f64 * t31920 * t31923 - 0.8673628188205199462e0_f64 * t31928 * t1035 * t1043 * t1089 + 0.11423947533020470523e1_f64 * t31934 * t31937 - 0.17347256376410398924e1_f64 * t31940 * t7174 + 0.17347256376410398924e1_f64 * t31943 * t1984 * t359 * t999 + 0.3718732920905101082e-3_f64 * t31950 * t31953;
    (t31951, t31953, t31956)
}
