//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1666/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1666<F: Float>(t225: F, t45384: F, t480: F, t12967: F, t12995: F, t3584: F, t1042: F, t1122: F, t1222: F, t12621: F, t1263: F, t12832: F, t12862: F, t12872: F, t12876: F, t12953: F, t12956: F, t12991: F, t17426: F, t17429: F, t17475: F, t17654: F, t17657: F, t17703: F, t17747: F, t17753: F, t17784: F, t3671: F, t371: F, t3711: F, t372: F, t3720: F, t43835: F, t44501: F, t44585: F, t44808: F, t45352: F, t45371: F, t45382: F, t482: F, t5352: F) -> (F, F, F) {
    let t45385 = t45384 * t225;
    let t45386 = t45385 * t480;
    let t45389 = t12967 * t12995;
    let t45391 = t3584 * t3584;
    let t45402 = F::cast_from(0.17149607247227894789e-2_f64) * t12956 * t12953 + F::cast_from(0.57165357490759649296e-3_f64) * t45352 + F::cast_from(0.57165357490759649296e-3_f64) * t3711 * t1042 * t1263 * t12621 * t1122 - F::cast_from(0.77173232612525526552e-2_f64) * t17747 * t3720 * t44585 * t17703 + F::cast_from(0.12862205435420921092e-2_f64) * t17753 * t3720 * t44585 * t17784 - F::cast_from(0.34299214494455789578e-2_f64) * t17654 * t44808 * t17657 - F::cast_from(0.85748036236139473944e-3_f64) * t45371 * t3720 * t44501 * t5352 - F::cast_from(0.25724410870841842184e-2_f64) * t17429 * t12876 + F::cast_from(0.51448821741683684368e-2_f64) * t17426 * t12872 - F::cast_from(0.34299214494455789578e-2_f64) * t45382 - F::cast_from(0.51448821741683684368e-2_f64) * t45386 * t12991 + F::cast_from(0.34299214494455789578e-2_f64) * t45389 + F::cast_from(0.12862205435420921092e-2_f64) * t3671 * t371 * t372 * t482 * t45391 - F::new(7.0) / F::new(108.0) * t1222 * t17475 * t43835 - F::cast_from(0.25724410870841842184e-2_f64) * t12832 * t12862;
    (t45385, t45391, t45402)
}
