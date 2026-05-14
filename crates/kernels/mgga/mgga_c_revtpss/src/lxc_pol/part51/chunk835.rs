//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 835/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk835<F: Float>(t1096: F, t373: F, t372: F, t371: F, t1035: F, t1043: F, t1089: F, t1984: F, t31885: F, t31888: F, t31891: F, t31894: F, t31897: F, t31899: F, t31903: F, t31905: F, t31909: F, t31913: F, t31916: F, t31920: F, t31923: F, t31928: F, t31934: F, t31937: F, t31940: F, t31943: F, t31950: F, t359: F, t7174: F, t8502: F, t8524: F, t999: F) -> (F, F, F) {
    let t31951 = t373 * t1096;
    let t31952 = t372 * t31951;
    let t31953 = t371 * t31952;
    let t31956 = -t31885 - 0.28234466758480466999e-3 * t8502 * t31888 + 0.11423947533020470523e1 * t31891 * t31894 + 0.17135921299530705785e1 * t31897 * t31899 - 0.17135921299530705785e1 * t31903 * t31905 - 0.1859366460452550541e-3 * t31909 * t8524 + 0.56468933516960933998e-3 * t31913 * t31916 - 0.56468933516960933998e-3 * t31920 * t31923 - 0.8673628188205199462e0 * t31928 * t1035 * t1043 * t1089 + 0.11423947533020470523e1 * t31934 * t31937 - 0.17347256376410398924e1 * t31940 * t7174 + 0.17347256376410398924e1 * t31943 * t1984 * t359 * t999 + 0.3718732920905101082e-3 * t31950 * t31953;
    (t31951, t31953, t31956)
}
