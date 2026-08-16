//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 945/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk945(t1065: f64, t385: f64, t906: f64, t32015: f64, t3104: f64, t72: f64, t3117: f64, t1028: f64, t1047: f64, t1096: f64, t1984: f64, t31891: f64, t31908: f64, t31961: f64, t31966: f64, t31972: f64, t31975: f64, t31978: f64, t31981: f64, t31986: f64, t31992: f64, t31994: f64, t32000: f64, t32003: f64, t32006: f64, t32010: f64, t32014: f64, t359: f64, t8504: f64, t8508: f64, t8517: f64, t8522: f64, t988: f64) -> (f64, f64, f64, f64, f64) {
    let t32016 = t1065 * t385;
    let t32017 = t32016 * t906;
    let t32018 = t32015 * t32017;
    let t32021 = t3104 * t72;
    let t32022 = t32021 * t3117;
    let t32025 = 0.57119737665102352616e0_f64 * t31908 * t8517 - 0.17135921299530705785e1_f64 * t31891 * t31961 - 0.15058382271189582399e-2_f64 * t31966 * t8504 + t31972 + 0.28234466758480466999e-3_f64 * t31975 * t31978 + 0.17347256376410398924e1_f64 * t8508 * t31981 * t359 * t1096 - 0.17347256376410398924e1_f64 * t31986 * t1984 * t359 * t988 - 0.12395776403017003607e-3_f64 * t31992 * t31994 - 0.3718732920905101082e-3_f64 * t32000 * t1047 - 0.5578099381357651623e-3_f64 * t32003 * t32006 + 0.5578099381357651623e-3_f64 * t32010 * t1028 + 0.18822977838986977999e-3_f64 * t32014 * t32018 + 0.99166211224136028853e-3_f64 * t8522 * t32022;
    (t32016, t32017, t32021, t32022, t32025)
}
