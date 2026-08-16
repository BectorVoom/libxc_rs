//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1666/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1666(t225: f64, t45384: f64, t480: f64, t12967: f64, t12995: f64, t3584: f64, t1042: f64, t1122: f64, t1222: f64, t12621: f64, t1263: f64, t12832: f64, t12862: f64, t12872: f64, t12876: f64, t12953: f64, t12956: f64, t12991: f64, t17426: f64, t17429: f64, t17475: f64, t17654: f64, t17657: f64, t17703: f64, t17747: f64, t17753: f64, t17784: f64, t3671: f64, t371: f64, t3711: f64, t372: f64, t3720: f64, t43835: f64, t44501: f64, t44585: f64, t44808: f64, t45352: f64, t45371: f64, t45382: f64, t482: f64, t5352: f64) -> (f64, f64, f64) {
    let t45385 = t45384 * t225;
    let t45386 = t45385 * t480;
    let t45389 = t12967 * t12995;
    let t45391 = t3584 * t3584;
    let t45402 = 0.17149607247227894789e-2_f64 * t12956 * t12953 + 0.57165357490759649296e-3_f64 * t45352 + 0.57165357490759649296e-3_f64 * t3711 * t1042 * t1263 * t12621 * t1122 - 0.77173232612525526552e-2_f64 * t17747 * t3720 * t44585 * t17703 + 0.12862205435420921092e-2_f64 * t17753 * t3720 * t44585 * t17784 - 0.34299214494455789578e-2_f64 * t17654 * t44808 * t17657 - 0.85748036236139473944e-3_f64 * t45371 * t3720 * t44501 * t5352 - 0.25724410870841842184e-2_f64 * t17429 * t12876 + 0.51448821741683684368e-2_f64 * t17426 * t12872 - 0.34299214494455789578e-2_f64 * t45382 - 0.51448821741683684368e-2_f64 * t45386 * t12991 + 0.34299214494455789578e-2_f64 * t45389 + 0.12862205435420921092e-2_f64 * t3671 * t371 * t372 * t482 * t45391 - 7.0_f64 / 108.0_f64 * t1222 * t17475 * t43835 - 0.25724410870841842184e-2_f64 * t12832 * t12862;
    (t45385, t45391, t45402)
}
