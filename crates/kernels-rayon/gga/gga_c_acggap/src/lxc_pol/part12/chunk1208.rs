//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1208/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1208(t35909: f64, t35911: f64, t35913: f64, t35915: f64, t35917: f64, t35919: f64, t31629: f64, t31632: f64, t31634: f64, t31638: f64, t31640: f64, t31644: f64, t31646: f64, t31658: f64, t31660: f64, t32891: f64, t35907: f64) -> f64 {
    let t37777 = 0.916875e-1_f64 * t35909;
    let t37778 = 0.916875e-1_f64 * t35911;
    let t37779 = 0.61125e-1_f64 * t35913;
    let t37780 = 0.61125e-1_f64 * t35915;
    let t37781 = 0.34299214494455789578e-2_f64 * t35917;
    let t37782 = 0.34299214494455789578e-2_f64 * t35919;
    let t37785 = 0.25724410870841842184e-1_f64 * t31629 - 0.16006300097412701803e-1_f64 * t31632 - 0.12862205435420921092e-1_f64 * t31634 + 0.94344276868812456206e-2_f64 * t31638 - 0.17149607247227894789e-1_f64 * t31640 - 0.45351183609335988442e-1_f64 * t31644 - 0.64025200389650807212e-1_f64 * t31646 - 0.21437009059034868486e-2_f64 * t35907 + t37777 + t37778 + t37779 + t37780 - t37781 + t37782 - 0.16772315887788881103e-1_f64 * t31658 + 0.18868855373762491241e-2_f64 * t31660 + t32891;
    t37785
}
