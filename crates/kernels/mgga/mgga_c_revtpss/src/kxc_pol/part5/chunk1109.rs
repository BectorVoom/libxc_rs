//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1109/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1109<F: Float>(t3022: F, t6219: F, t6223: F, t2986: F, t6205: F, t974: F, t981: F, t4708: F, t4724: F, t3336: F, t6396: F, t6184: F, t964: F, t19021: F, t973: F, t11461: F, t11554: F, t15343: F, t1634: F, t19029: F, t19031: F, t19058: F, t19060: F, t19062: F, t2982: F, t4685: F, t6190: F, t6206: F, t6209: F, t965: F) -> (F, F, F, F, F, F) {
    let t19143 = 0.11696447245269292414e1 * t3022 * t6219;
    let t19145 = 0.5848223622634646207e0 * t3022 * t6223;
    let t19146 = t2986 * t6205;
    let t19147 = t19146 * t974;
    let t19149 = 0.11696447245269292414e1 * t981 * t19147;
    let t19150 = t4724 * t4708;
    let t19152 = 0.23392894490538584828e1 * t981 * t19150;
    let t19153 = t6396 * t3336;
    let t19156 = t6184 * t964;
    let t19167 = t19021 * t973;
    let t19172 = t19029 - t19031 + 0.5848223622634646207e0 * t19156 * t974 + 0.11696447245269292414e1 * t15343 * t1634 + 0.11696447245269292414e1 * t4685 * t4708 - 0.11696447245269292414e1 * t11554 * t6190 + 0.5848223622634646207e0 * t2982 * t6206 + 0.5848223622634646207e0 * t965 * t19167 + 0.17315859105681463759e2 * t11461 * t6209 - t19058 - t19060 - t19062;
    (t19143, t19145, t19149, t19152, t19153, t19172)
}
