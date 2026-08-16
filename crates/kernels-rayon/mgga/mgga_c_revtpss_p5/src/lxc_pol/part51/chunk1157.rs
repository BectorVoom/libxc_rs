//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1157/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1157(t126004: f64, t126433: f64, t1646: f64, t999: f64, t385: f64, t905: f64, t1469: f64, t988: f64, t11922: f64, t31975: f64, t33760: f64, t120201: f64, t120285: f64, t120301: f64, t120306: f64, t120321: f64, t120376: f64, t120429: f64, t120460: f64, t120466: f64, t120476: f64, t1665: f64, t3092: f64, t31950: f64, t32000: f64, t32015: f64, t33761: f64, t371: f64, t372: f64, t373: f64, t4781: f64, t4869: f64, t4873: f64, t5015: f64, t99566: f64) -> (f64, f64, f64, f64) {
    let t126434 = t126004 + t126433;
    let t126442 = t1646 * t999;
    let t126447 = t385 * t905;
    let t126448 = t1469 * t988;
    let t126460 = t31975 * t11922 * t33760;
    let t126471 = -0.29749863367240808656e-2_f64 * t120476 * t1665 + 0.3718732920905101082e-3_f64 * t31950 * t371 * t372 * t373 * t5015 - 0.112937867033921868e-2_f64 * t120466 * t32015 * t120306 * t126442 + 0.37645955677973955998e-3_f64 * t120376 * t3092 * t126447 * t126448 - 0.37645955677973955998e-3_f64 * t120321 * t3092 * t126447 * t4873 - 0.15058382271189582399e-2_f64 * t120285 * t33761 + 0.18822977838986977999e-3_f64 * t126460 + 0.5578099381357651623e-3_f64 * t120301 * t1665 - 0.28234466758480466999e-3_f64 * t120429 * t120460 * t4781 * t99566 + 0.12548651892657985333e-3_f64 * t120201 - 0.3718732920905101082e-3_f64 * t32000 * t4869;
    (t126434, t126442, t126448, t126471)
}
