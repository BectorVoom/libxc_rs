//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1172/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1172(t100533: f64, t31999: f64, t8513: f64, t1047: f64, t1096: f64, t120281: f64, t120513: f64, t120602: f64, t120664: f64, t120676: f64, t120696: f64, t126725: f64, t126903: f64, t1984: f64, t27627: f64, t27652: f64, t31892: f64, t31897: f64, t31903: f64, t31943: f64, t31959: f64, t31986: f64, t33791: f64, t33792: f64, t359: f64, t4742: f64, t4772: f64, t4976: f64, t7135: f64, t7821: f64, t8521: f64, t8524: f64, t999: f64) -> f64 {
    let t127016 = t8513 * t100533 * t31999;
    let t127035 = -0.51407763898592117355e1_f64 * t120602 * t31892 * t33791 * t999 - 0.10038921514126388266e-2_f64 * t120696 + 0.17135921299530705785e1_f64 * t120281 * t33792 - 0.1859366460452550541e-3_f64 * t126725 * t8521 * t8524 - 0.17347256376410398924e1_f64 * t120513 * t126903 * t27652 + 0.17347256376410398924e1_f64 * t120664 * t126903 * t4976 - 0.17347256376410398924e1_f64 * t120676 * t27627 - 0.3718732920905101082e-3_f64 * t127016 * t1047 - 0.3427184259906141157e1_f64 * t31903 * t31892 * t7821 * t7135 - 0.51407763898592117355e1_f64 * t31897 * t31959 * t33791 * t1096 + 0.17347256376410398924e1_f64 * t31943 * t1984 * t359 * t4772 - 0.17347256376410398924e1_f64 * t31986 * t1984 * t359 * t4742;
    t127035
}
