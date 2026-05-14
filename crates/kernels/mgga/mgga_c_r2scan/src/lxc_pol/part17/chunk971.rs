//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 971/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk971<F: Float>(t10648: F, t10958: F, t10971: F, t10962: F, t37599: F, t37822: F, t37833: F, t37919: F, t38001: F, t38054: F, t38068: F, t38130: F, t38143: F, t38164: F, t38175: F, t38189: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t38359 = t10648 * t10971 * t10958;
    let t38362 = t10648 * t10971 * t10962;
    let t38452 = 0.42952285777298855708e-4 * t37599;
    let t38528 = 0.14224135994914204065e1 * t37822;
    let t38532 = 0.17888640988868435534e-2 * t37833;
    let t38568 = 0.18496169001454677638e1 * t37919;
    let t38597 = 0.31806003678208078381e-2 * t38001;
    let t38617 = 0.39552774754617995815e1 * t38054;
    let t38622 = 0.19634394786159580877e0 * t38068;
    let t38646 = 0.28914548798370980346e-4 * t38130;
    let t38649 = 0.23159605016379617484e1 * t38143;
    let t38657 = 0.51410067763503603055e-4 * t38164;
    let t38661 = 0.34909953929791734801e0 * t38175;
    let t38666 = 0.46160609703545424213e1 * t38189;
    (t38359, t38362, t38452, t38528, t38532, t38568, t38597, t38617, t38622, t38646, t38649, t38657, t38661, t38666)
}
