//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 848/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk848<F: Float>(t1065: F, t385: F, t906: F, t32015: F, t3104: F, t72: F, t3117: F, t1028: F, t1047: F, t1096: F, t1984: F, t31891: F, t31908: F, t31961: F, t31966: F, t31972: F, t31975: F, t31978: F, t31981: F, t31986: F, t31992: F, t31994: F, t32000: F, t32003: F, t32006: F, t32010: F, t32014: F, t359: F, t8504: F, t8508: F, t8517: F, t8522: F, t988: F) -> (F, F, F, F, F) {
    let t32016 = t1065 * t385;
    let t32017 = t32016 * t906;
    let t32018 = t32015 * t32017;
    let t32021 = t3104 * t72;
    let t32022 = t32021 * t3117;
    let t32025 = 0.57119737665102352616e0 * t31908 * t8517 - 0.17135921299530705785e1 * t31891 * t31961 - 0.15058382271189582399e-2 * t31966 * t8504 + t31972 + 0.28234466758480466999e-3 * t31975 * t31978 + 0.17347256376410398924e1 * t8508 * t31981 * t359 * t1096 - 0.17347256376410398924e1 * t31986 * t1984 * t359 * t988 - 0.12395776403017003607e-3 * t31992 * t31994 - 0.3718732920905101082e-3 * t32000 * t1047 - 0.5578099381357651623e-3 * t32003 * t32006 + 0.5578099381357651623e-3 * t32010 * t1028 + 0.18822977838986977999e-3 * t32014 * t32018 + 0.99166211224136028853e-3 * t8522 * t32022;
    (t32016, t32017, t32021, t32022, t32025)
}
