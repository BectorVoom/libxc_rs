//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1164/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1164(t11921: f64, t247: f64, t31920: f64, t33754: f64, t4930: f64, t8513: f64, t1035: f64, t1042: f64, t1043: f64, t1089: f64, t120256: f64, t120387: f64, t120419: f64, t120443: f64, t120447: f64, t120558: f64, t120584: f64, t120654: f64, t1669: f64, t19639: f64, t31928: f64, t31966: f64, t33756: f64, t33770: f64, t33796: f64, t33797: f64, t33803: f64, t33832: f64, t4866: f64, t8517: f64) -> (f64, f64) {
    let t126719 = t31920 * t247 * t11921 * t33754;
    let t126725 = t8513 * t4930;
    let t126741 = 0.3718732920905101082e-3_f64 * t120256 * t1042 * t1669 * t19639 + 0.15058382271189582399e-2_f64 * t31966 * t33770 + 0.30116764542379164799e-2_f64 * t120419 * t33756 - 0.37645955677973955999e-3_f64 * t126719 - 0.17135921299530705785e1_f64 * t120584 * t33797 - 0.12395776403017003607e-3_f64 * t120387 * t33832 + 0.57119737665102352616e0_f64 * t126725 * t8517 - 0.8673628188205199462e0_f64 * t31928 * t1035 * t4866 * t1089 - 0.3427184259906141157e1_f64 * t120558 * t33796 * t1043 * t1089 - 0.3427184259906141157e1_f64 * t120654 * t33803 * t1043 * t1089 + 0.66110807482757352571e-3_f64 * t120443 + t120447;
    (t126725, t126741)
}
