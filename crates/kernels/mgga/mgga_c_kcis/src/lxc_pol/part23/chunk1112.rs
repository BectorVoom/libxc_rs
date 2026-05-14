//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1112/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1112<F: Float>(t27357: F, t5440: F, t98359: F, t28347: F, t94246: F, t27369: F, t1464: F, t28360: F, t94216: F, t27364: F, t28382: F, t7908: F, t94227: F, t94353: F, t98146: F, t98344: F, t98347: F, t98350: F, t98353: F, t98357: F) -> (F, F, F, F, F) {
    let t98361 = t98359 * t5440 * t27357;
    let t98364 = t94246 * t28347;
    let t98365 = t27369 * t98364;
    let t98370 = t1464 * t94216 * t28360;
    let t98373 = t1464 * t27364 * t28382;
    let t98375 = -0.44218518518518518517e-2 * t98344 + 0.66327777777777777776e-2 * t98347 - 0.33163888888888888888e-2 * t98350 + 0.66327777777777777776e-2 * t98353 + 0.15445601851851851852e-3 * t94353 + 0.55273148148148148146e-2 * t98357 - 0.12367293402777777778e-3 * t94227 * t98361 - 0.61836467013888888888e-4 * t98365 + 0.13901041666666666667e-2 * t7908 * t98146 - 0.33163888888888888888e-2 * t98370 - 0.11054629629629629629e-2 * t98373;
    (t98361, t98364, t98370, t98373, t98375)
}
