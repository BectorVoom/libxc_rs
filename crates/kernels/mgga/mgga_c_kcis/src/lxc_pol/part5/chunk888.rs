//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 888/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk888<F: Float>(t9725: F, t250: F, t253: F, t3106: F, t242: F, t245: F, t255: F, t2984: F, t929: F, t244: F, t260: F, t2987: F, t2937: F, t926: F, t270: F, t3030: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9726 = 0.93011851851851851854e0 * t9725;
    let t9728 = t250 * t3106 * t253;
    let t9729 = 0.36514074074074074075e0 * t9728;
    let t9736 = 28.0 / 27.0 * t9725;
    let t9752 = 1.0/pow_3_2(t242);
    let t9758 = 1.0 / t245 / t255 / 4.0;
    let t9767 = 1.0 / t2984 / t929;
    let t9768 = t244 * t9767;
    let t9770 = 1.0 / t2987 / t260;
    let t9775 = 0.28842592592592592592e-1 * t9725;
    let t9790 = 0.55403703703703703703e-1 * t9725;
    let t9804 = t926 * t2937;
    let t9825 = 1.0 / t3030 / t270;
    (t9726, t9728, t9729, t9736, t9752, t9758, t9768, t9770, t9775, t9790, t9804, t9825)
}
