//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1157/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1157<F: Float>(t126004: F, t126433: F, t1646: F, t999: F, t385: F, t905: F, t1469: F, t988: F, t11922: F, t31975: F, t33760: F, t120201: F, t120285: F, t120301: F, t120306: F, t120321: F, t120376: F, t120429: F, t120460: F, t120466: F, t120476: F, t1665: F, t3092: F, t31950: F, t32000: F, t32015: F, t33761: F, t371: F, t372: F, t373: F, t4781: F, t4869: F, t4873: F, t5015: F, t99566: F) -> (F, F, F, F) {
    let t126434 = t126004 + t126433;
    let t126442 = t1646 * t999;
    let t126447 = t385 * t905;
    let t126448 = t1469 * t988;
    let t126460 = t31975 * t11922 * t33760;
    let t126471 = -F::new(0.29749863367240808656e-2) * t120476 * t1665 + F::new(0.3718732920905101082e-3) * t31950 * t371 * t372 * t373 * t5015 - F::new(0.112937867033921868e-2) * t120466 * t32015 * t120306 * t126442 + F::new(0.37645955677973955998e-3) * t120376 * t3092 * t126447 * t126448 - F::new(0.37645955677973955998e-3) * t120321 * t3092 * t126447 * t4873 - F::new(0.15058382271189582399e-2) * t120285 * t33761 + F::new(0.18822977838986977999e-3) * t126460 + F::new(0.5578099381357651623e-3) * t120301 * t1665 - F::new(0.28234466758480466999e-3) * t120429 * t120460 * t4781 * t99566 + F::new(0.12548651892657985333e-3) * t120201 - F::new(0.3718732920905101082e-3) * t32000 * t4869;
    (t126434, t126442, t126448, t126471)
}
